use rand::thread_rng;
use crate::IpcMessage;
use num_bigint::{RandBigInt,BigUint};
use std::{
    os::unix::net::{UnixStream,UnixListener},
    io::{BufRead, Write, BufReader},
    sync::{Arc,Mutex},
    path::Path,
    thread, 
    fs, 
};
use crate::ipc::{
    wm::{
        prime::generate_prime,
        handler::handler
    },
    message::{
        PunkIpcMessage,
        DHClientDataMessage,
        DHServerDataMessage,
        IPC_PUNK_NAME,
        IPC_PUNK_DOCK_NAME,
        IpcMessageEncrypted,
    },
};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305,
    Key,
    Nonce,
};
use sha2::{Digest, Sha256};
use calloop::channel::{channel, Event, Sender};
use crate::apphandle::get_api_ipc;

pub const SOCKET_PATH_PUNK: &str = "/tmp/{user}_punk.sock";
pub const SOCKET_PATH_PUNK_DOCK: &str = "/tmp/{user}_punk_dock.sock";

pub struct PunkIPC{
    p: BigUint,
    g: BigUint,
    my_key: BigUint,
    my_pub_key: BigUint,
    your_pub_key: BigUint,
    key: BigUint
}

impl PunkIPC{
    pub fn new()->Self{
        let p = generate_prime(512);
        let g = BigUint::from(2u32);
        let mut rng = thread_rng();
        let my_key = rng.gen_biguint(256);
        let pub_key = g.modpow(&my_key, &p);

        Self{
            p,
            g,
            my_key,
            my_pub_key:pub_key,
            your_pub_key:BigUint::from(0u32),
            key: BigUint::from(0u32)
        }
    }

    pub fn send(&self,msg: IpcMessage) -> bool {
        let message = match serde_json::to_string(&msg){
            Ok(t)=> t,
            Err(_)=> return false
        };

        let (encrypted, nonce) = self.encrypt_message(message);

        let message_json = match serde_json::to_value(&IpcMessageEncrypted::new(encrypted, nonce)){
            Ok(t)=> t,
            Err(_)=> return false
        };

        let socket_msg = PunkIpcMessage::new_punk_ipc_message(message_json);

        let mut stream = match UnixStream::connect(&socket_path()){
            Ok(t)=> t,
            Err(_)=> return false
        };

        let json = match serde_json::to_string(&socket_msg){
            Ok(t)=> t,
            Err(_)=> return false
        };

        match stream.write(format!("{}\n", json).as_bytes()){
            Ok(_)=> return true,
            Err(_)=> return false
        };
    }

    pub fn my_pub_key(&self)->BigUint{self.my_pub_key.clone()}
    pub fn p(&self)->BigUint{self.p.clone()}
    pub fn g(&self)->BigUint{self.g.clone()}
    pub fn key(&self)->BigUint{self.key.clone()}

    pub fn set_your_pub_key(&mut self, key: BigUint){
        self.your_pub_key = key;
    }
    pub fn generate_key(&mut self){
        self.key = self.your_pub_key.modpow(&self.my_key, &self.p);
    }

    pub fn link(&self)->bool{
        let msg = DHClientDataMessage::from_punk_ipc(self);
        let socket_msg = PunkIpcMessage::new_punk_ipc_connection(msg);
        let mut stream = match UnixStream::connect(&socket_path()){
            Ok(t)=> t,
            Err(_)=> return false
        };

        let json = match serde_json::to_string(&socket_msg){
            Ok(t)=> t,
            Err(_)=> return false
        };

        match stream.write(format!("{}\n", json).as_bytes()){
            Ok(_)=> return true,
            Err(_)=> return false
        };
    }

    fn encrypt_message(&self, message: String) -> (Vec<u8>, [u8; 12]) {
        let key_bytes = self.key().to_bytes_be();
        let hash = Sha256::digest(&key_bytes);

        let key = Key::from_slice(&hash);

        let cipher = ChaCha20Poly1305::new(key);

        let nonce_bytes = rand::random::<[u8; 12]>();

        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, message.as_bytes())
            .unwrap();

        (ciphertext, nonce_bytes)
    }


    pub fn decrypt_message(&self,msg: IpcMessageEncrypted,) -> String {
        let key_bytes = self.key().to_bytes_be();
        let hash = Sha256::digest(&key_bytes);

        let key = Key::from_slice(&hash);

        let cipher = ChaCha20Poly1305::new(key);

        let nonce = Nonce::from_slice(&msg.nonce);

        let plaintext = cipher.decrypt(nonce, msg.encrypted.as_slice()).unwrap();
        String::from_utf8(plaintext).unwrap()
    }


    pub fn start_ipc_server(ipc: Arc<Mutex<PunkIPC>>) -> std::io::Result<()> {

        let path_socket = socket_path_dock();
        let path = Path::new(&path_socket);

        let _ = fs::remove_file(path);

        let listener = UnixListener::bind(path)?;

        thread::spawn(move || {

            for stream in listener.incoming() {

                let stream = match stream {
                    Ok(s) => s,
                    Err(err) => {
                        println!("accept error: {:?}", err);
                        continue;
                    }
                };

                let ipc = ipc.clone();

                thread::spawn(move || {

                    let mut reader = BufReader::new(stream);

                    loop {

                        let mut line = String::new();

                        match reader.read_line(&mut line) {

                            Ok(0) => {break;}

                            Ok(_) => {

                                let line = line.trim();

                                if let Ok(msg) = serde_json::from_str::<PunkIpcMessage>(line) {
                                    if msg.sender == IPC_PUNK_DOCK_NAME{
                                        break;
                                    }
                                    if msg.sender == IPC_PUNK_NAME {
                                        if msg.type_ == String::from("DOCK CONNECTION D/H") {
                                            let data: DHServerDataMessage =serde_json::from_value(msg.data).unwrap();
                                            let mut ipc = ipc.lock().unwrap();
                                            ipc.set_your_pub_key(data.pub_key);
                                            ipc.generate_key();
                                        }else if msg.type_ == String::from("DOCK MESSAGE ENCODE") {
                                            let data: IpcMessageEncrypted = serde_json::from_value(msg.data).unwrap();
                                            let mut ipc = ipc.lock().unwrap();
                                            let message_json = ipc.decrypt_message(data);
                                            let message:IpcMessage = serde_json::from_str(&message_json).unwrap();
                                            handler(message);
                                        }
                                    }
                                }
                            }

                            Err(err) => {
                                println!("read error {:?}", err);
                                break;
                            }
                        }
                    }
                });
            }
        });

        Ok(())
    }

}

pub fn socket_path() -> String {
    let user = std::env::var("USER").unwrap_or_default();
    SOCKET_PATH_PUNK.replace("{user}", &user)
}

pub fn socket_path_dock() -> String {
    let user = std::env::var("USER").unwrap_or_default();
    SOCKET_PATH_PUNK_DOCK.replace("{user}", &user)
}


