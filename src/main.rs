use clap::{App, Arg};
use cli_table::{print_stdout, Style, Table, WithTitle};
use ipdb::Reader;

fn main() {
    let matches = App::new("ip-cli")
        .about("simple ip find tool, use ipip free db")
        .version("v0.0.1")
        .arg(Arg::from_usage("<arg>... 'IP address'").required(true))
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

    let ips = matches.values_of_lossy("arg").unwrap();
    let mut items: Vec<IpAddr> = Vec::with_capacity(ips.len() as usize);
    for ip in ips {
        let r = ipdb.find(&ip, "CN");
        if let Ok(r) = r {
            let address = r
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let item = IpAddr {
                ip,
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
            if i.len() == 0 {
                continue;
            }
            s.push_str(i);
            s.push_str(", ");
        }
        if s.len() > 0 {
            s = (&s[..s.len() - 2]).to_string();
        }
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Address").field(&self.0).finish()
    }
}
