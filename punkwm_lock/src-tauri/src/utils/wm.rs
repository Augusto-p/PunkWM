use punkwm_lock_lib::print_in_tty;
use std::process::Command;
use std::os::unix::process::CommandExt;
use nix::unistd::{setuid, setgid, setsid, Uid, User};
    


pub fn launch_wm_with_env(user_uid: u32, wm_path: &str) -> ! {
    let user = User::from_uid(Uid::from_raw(user_uid))
        .expect("no user lookup")
        .expect("user missing");

    let gid = user.gid;
    let home = user.dir.clone();
    let name = user.name.clone();
    let shell = user.shell.clone();

    let mut cmd = std::process::Command::new(wm_path);
     // ✅ entorno — acá sí
    cmd.env("HOME", &home)
       .env("USER", &name)
       .env("LOGNAME", &name)
       .env("SHELL", &shell)
       .env("PATH", "/usr/local/sbin:/usr/local/bin:/usr/bin:/bin")
       .env("XDG_RUNTIME_DIR", format!("/run/user/{}", user_uid))
        .env("XDG_SESSION_TYPE", "x11")
        .env("XDG_SESSION_CLASS", "user")
        .env("XDG_CURRENT_DESKTOP", "PunkWM")
        .env("XDG_SESSION_DESKTOP", "PunkWM")
       .current_dir(&home);
    unsafe {
        cmd.pre_exec(move || {
         
            // nueva sesión (recomendado en DM)
            setgid(gid)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            setuid(Uid::from_raw(user_uid))
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            Ok(())
        });
    }

    let err = cmd.exec();

    panic!("EXEC FALLÓ: {:?}", err);
}

