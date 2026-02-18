use pam::Client;

pub fn login_auth(user : &str, password : &str) -> bool{
    let mut client = match Client::with_password("system-auth"){
        Ok(cli) => cli,
        Err(_) => return false
    };
    client.conversation_mut().set_credentials(user, password);
    match client.authenticate(){
        Ok(_)=> return true,
        Err(_)=> return false
    };
}
