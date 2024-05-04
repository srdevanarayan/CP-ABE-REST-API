#[macro_use]
extern crate rocket;
use rabe::schemes::ac17::*;
use rabe::utils::policy::pest::PolicyLanguage;
use serde::Deserialize;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use rocket_cors;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::log::private::debug;
use rocket::{Request, Response};

lazy_static! {
    static ref INITIALIZED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

static mut PK: Option<Ac17PublicKey> = None;
static mut MSK: Option<Ac17MasterKey> = None;


#[derive(Deserialize)]
struct Attributes {
    attributes: Vec<String>,
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[derive(Deserialize)]
struct EncryptionData {
    message: String,
    access_policy: String,
}

#[derive(Deserialize)]
struct DecryptionData {
    secret_key: Ac17CpSecretKey,
    ciphertext: Ac17CpCiphertext,
}

#[get("/setup")]
fn syssetup(initialized: &State<Arc<Mutex<bool>>>) -> String {
    let mut is_initialized = initialized.lock().unwrap();
    if *is_initialized {
        String::from("System already initialized")
    } else {
        *is_initialized = true;
        unsafe {
            let (_pk, _msk) = setup();
            
            //println!("Type of _a1_key: {}", type_of(&_a1_key));
            PK = Some(_pk);
            MSK = Some(_msk);
        
        }
        String::from("System initialized")
    }
}

#[post("/generatekey", data = "<attributes>")]
fn generate_key(attributes: Json<Attributes>) -> Option<Json<Ac17CpSecretKey>> {
    unsafe {
        if let Some(msk) = &MSK {
            if let Ok(sk) = cp_keygen(&msk, &attributes.attributes) {
                // Print the secret key
                //println!("Generated secret key: {:?}", sk);
                return Some(Json(sk));
            } else {
                println!("Failed to generate secret key");
            }
        } else {
            println!("MSK is not initialized yet");
        }
    }
    
    println!("Received attributes: {:?}", attributes.attributes);
    None
}
#[post("/encrypt", data = "<data>")]
fn encrypt(data: Json<EncryptionData>) -> Result<Json<Ac17CpCiphertext>, String> {
    unsafe {
        if let Some(pk) = &PK {
            // Extract message and access policy from the JSON data
            let message = data.message.as_bytes(); // Convert message string to bytes
            let policy = &format!(r#"{}"#, data.access_policy); // Format the policy string

            // Encrypt the message using the provided access policy
            if let Ok(ct) = cp_encrypt(pk, policy, message, PolicyLanguage::HumanPolicy) {
                // Return the encrypted message
                return Ok(Json(ct));
            } else {
                let error_message = "Failed to encrypt message";
                println!("{}", error_message);
                return Err(error_message.to_string());
            }
        } else {
            let error_message = "PK is not initialized yet";
            println!("{}", error_message);
            return Err(error_message.to_string());
        }
    }
}


#[post("/decrypt", data = "<data>")]
fn decrypt(data: Json<DecryptionData>) -> Result<String, String> {
    // Extract the secret key and ciphertext from the JSON data
    let sk = &data.secret_key;
    let ciphertext = &data.ciphertext;

    // Decrypt the ciphertext using the provided secret key
    match cp_decrypt(sk, ciphertext) {
        Ok(plaintext) => {
            // Convert the plaintext bytes to a UTF-8 string
            let plaintext_string = String::from_utf8_lossy(&plaintext);
            // Return the decrypted message
            Ok(plaintext_string.into_owned())
        },
        Err(error) => {
            let error_message = format!("Failed to decrypt message: {:?}", error);
            println!("{}", error_message);
            Err(error_message)
        }
    }
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Cors)
        .manage(INITIALIZED.clone())
        .mount("/", routes![syssetup, generate_key, encrypt, decrypt,all_options])
}