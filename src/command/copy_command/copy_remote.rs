use std::fs::File;
use std::io::{self, Error, ErrorKind, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::time::Duration;
use regex::Regex;
use rpassword::read_password;
use ssh2::Session;
use crate::command::copy_command::copy::CopyArgs;

const CONNECTION_TIME_OUT_SECONDS: u64 = 3;

pub fn copy(args: &CopyArgs) -> io::Result<()> {
    if is_remote_path(&args.from) {
        scp_copy_from_remote(&args.from, &args.to)?
    } else if is_remote_path(&args.to) {
        scp_send_to_remote(&args.from, &args.to)?
    } else {
        return Err(Error::new(ErrorKind::Other, "Both paths are local or invalid"));
    }
    Ok(())
}

fn is_remote_path(path: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9_.-]+:/.*$").unwrap();
    re.is_match(path)
}

fn scp_copy_from_remote(from: &str, to: &str) -> io::Result<()> {
    let remote_info = RemoteInfo::from_str(from).ok_or_else(|| Error::new(ErrorKind::Other, "Invalid remote path"))?;

    let session = get_session(&remote_info)?;

    let (mut remote_file, _) = session.scp_recv(&remote_info.path)?;
    let mut local_file = File::create(to)?;
    io::copy(&mut remote_file, &mut local_file)?;
    Ok(())
}

fn scp_send_to_remote(from: &str, to: &str) -> io::Result<()> {
    let remote_info = RemoteInfo::from_str(to).expect("Invalid remote path");

    let session = get_session(&remote_info)?;
    let mut local_file = File::open(from)?;
    let metadata = local_file.metadata()?;
    let file_size = metadata.len();

    let mut remote_file = session.scp_send(Path::new(&remote_info.path), 0o644, file_size, None)?;
    io::copy(&mut local_file, &mut remote_file)?;
    Ok(())
}

fn get_session(remote_info: &RemoteInfo) -> io::Result<Session> {
    let addr = format!("{}:22", remote_info.host).to_socket_addrs()?.next().expect("Unable to resolve host");
    let tcp = TcpStream::connect_timeout(&addr, Duration::new(CONNECTION_TIME_OUT_SECONDS, 0))?;
    tcp.set_read_timeout(Some(Duration::new(CONNECTION_TIME_OUT_SECONDS, 0)))?;
    tcp.set_write_timeout(Some(Duration::new(CONNECTION_TIME_OUT_SECONDS, 0)))?;

    let mut session = Session::new().expect("Failed to create SSH session");
    session.set_tcp_stream(tcp);
    session.handshake()?;

    print!("Password: ");
    io::stdout().flush()?; // print! 호출 후 즉시 flush를 호출하여 프롬프트를 화면에 출력합니다.
    let password = read_password()?;
    println!("{}", password);

    session.userauth_password(&remote_info.user, &password)?;

    if !session.authenticated() {
        return Err(Error::new(ErrorKind::PermissionDenied, "Authentication failed"));
    }

    Ok(session)
}

struct RemoteInfo {
    user: String,
    host: String,
    path: PathBuf,
}

impl RemoteInfo {
    fn from_str(s: &str) -> Option<Self> {
        let re = Regex::new(r"^([a-zA-Z0-9_.+-]+)@([a-zA-Z0-9_.-]+):/(.*)$").unwrap();
        re.captures(s).map(|caps| {
            RemoteInfo {
                user: caps.get(1).unwrap().as_str().to_string(),
                host: caps.get(2).unwrap().as_str().to_string(),
                path: PathBuf::from(caps.get(3).unwrap().as_str()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_remote_path() {
        let path = "moseoh@192.168.0.2:/path";
        assert_eq!(is_remote_path(path), true);
    }
}
