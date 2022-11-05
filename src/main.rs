use clap::{Arg, ArgAction, Command};
use cli_table::{print_stdout, Style, Table, WithTitle};
use ipdb::Reader;

fn main() {
    let arg = Arg::default()
        .id("arg")
        .required(true)
        .action(ArgAction::Set)
        .allow_hyphen_values(true)
        .help("IP Address");

    let matches = Command::new("ip-cli")
        .about("simple ip find tool, use ipip free db")
        .version("v0.0.1")
        .arg(arg)
        .get_matches();

    let mut file = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("ipipfree.ipdb");

    if !file.exists() {
        file = std::env::current_dir()
            .unwrap()
            .join("assets/ipipfree.ipdb");
    }

    let ipdb: Reader = Reader::open_file(file).unwrap();

    let ips = matches.get_many("arg").unwrap().collect::<Vec<&String>>();

    let mut items: Vec<IpAddr> = Vec::with_capacity(ips.len() as usize);
    for ip in ips {
        let r = ipdb.find(ip, "CN");
        if let Ok(r) = r {
            let address = r
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let item = IpAddr {
                ip: ip.to_owned(),
                address: Address(address),
            };
            items.push(item);
        }
    }
    print_stdout(items.with_title().dimmed(true)).unwrap();
}

#[derive(Debug, Table)]
struct IpAddr {
    #[table(title = "IP")]
    pub ip: String,
    #[table(title = "Address")]
    address: Address,
}

struct Address(Vec<String>);

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in self.0.iter() {
            if i.is_empty() {
                continue;
            }
            s.push_str(i);
            s.push_str(", ");
        }
        if !s.is_empty() {
            s = (s[..s.len() - 2]).to_string();
        }
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Address").field(&self.0).finish()
    }
}
