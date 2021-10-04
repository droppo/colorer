use crate::{
    core::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
    decorate,
};

pub struct Nmap;

impl Parser for Nmap {
    fn regexs(&self) -> Vec<ColorerRegex> {
        vec![
            // IP
            ColorerRegex::new(
                r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}",
                decorate!(Decoration::GreenFgBright),
                None,
            ),
            // port and protocol
            ColorerRegex::new(
                r"\d+/[a-zA-Z]+",
                decorate!(Decoration::YellowFgBright),
                None,
            ),
            // state
            ColorerRegex::new(
                r"(?<=\d+\/[a-z]+\s+)[^\s]+",
                decorate!(Decoration::GreenFgBright),
                Some(vec![
                    ("closed", decorate!(Decoration::RedFgBright)),
                    ("filtered", decorate!(Decoration::MagentaFgBright)),
                ]),
            ),
            // service
            ColorerRegex::new(
                r"(?<=\d+\/[a-z]+\s+[a-zA-Z]+\s+)[^\s]+",
                decorate!(Decoration::CyanFgBright),
                None,
            ),
            // version
            ColorerRegex::new(
                r"(?<=(\d+\/[a-z]+\s+[a-z]+\s+\S+\s*[^\S\r\n]))\S.*",
                decorate!(Decoration::MagentaFgBright),
                None,
            ),
            // host is up or down
            ColorerRegex::new(
                r"(?<=(Host is\s|Host seems\s))\w+",
                decorate!(Decoration::GreenFgBright),
                Some(vec![("down", decorate!(Decoration::RedFgBright))]),
            ),
            // Nmap scan report for
            ColorerRegex::new(
                r"Nmap scan report for",
                decorate!(Decoration::Underlined),
                None,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{
            decorator::Decoration,
            parser::{init_parser, reader_handler},
        },
        decorate,
    };

    #[test]
    fn nmap_host_up() {
        let input = "Nmap scan report for 192.168.0.119
Host is up (0.020s latency).
Not shown: 994 closed ports
PORT     STATE SERVICE  VERSION
22/tcp  filtered  ssh      OpenSSH 8.3 (protocol 2.0)
53/tcp   open  domain   (generic dns response: NOTIMP)
80/tcp   open  http     lighttpd 1.4.53
443/tcp  open  ssl/http nginx
3000/tcp open  http     Gophish httpd"
            .to_string();

        let correct_output =
            format!("{underlined}Nmap scan report for{reset} {green}192.168.0.119{reset}
Host is {green}up{reset} (0.020s latency).
Not shown: 994 closed ports
PORT     STATE SERVICE  VERSION
{yellow}22/tcp{reset}  {purple}filtered{reset}  {cyan}ssh{reset}      {purple}OpenSSH 8.3 (protocol 2.0){reset}
{yellow}53/tcp{reset}   {green}open{reset}  {cyan}domain{reset}   {purple}(generic dns response: NOTIMP){reset}
{yellow}80/tcp{reset}   {green}open{reset}  {cyan}http{reset}     {purple}lighttpd 1.4.53{reset}
{yellow}443/tcp{reset}  {green}open{reset}  {cyan}ssl/http{reset} {purple}nginx{reset}
{yellow}3000/tcp{reset} {green}open{reset}  {cyan}http{reset}     {purple}Gophish httpd{reset}",
                    reset = decorate!(Decoration::Default),
                    yellow = decorate!(Decoration::YellowFgBright),
                    green = decorate!(Decoration::GreenFgBright),
                    cyan = decorate!(Decoration::CyanFgBright),
                    purple = decorate!(Decoration::MagentaFgBright),
                    underlined = decorate!(Decoration::Underlined)
            );

        let args: Vec<String> = vec![];
        assert_eq!(
            correct_output,
            reader_handler(input, &init_parser("nmap", &args).unwrap())
        );
    }

    #[test]
    fn nmap_host_down() {
        let input = "Starting Nmap 7.70 ( https://nmap.org ) at 2021-08-11 12:50 CEST
Note: Host seems down. If it is really up, but blocking our ping probes, try -Pn
Nmap done: 1 IP address (0 hosts up) scanned in 3.03 seconds
"
        .to_string();

        let correct_output = format!(
            "Starting Nmap 7.70 ( https://nmap.org ) at 2021-08-11 12:50 CEST
Note: Host seems {red}down{reset}. If it is really up, but blocking our ping probes, try -Pn
Nmap done: 1 IP address (0 hosts up) scanned in 3.03 seconds
",
            red = decorate!(Decoration::RedFgBright),
            reset = decorate!(Decoration::Default)
        );

        let args: Vec<String> = vec![];
        assert_eq!(
            correct_output,
            reader_handler(input, &init_parser("nmap", &args).unwrap())
        );
    }
}
