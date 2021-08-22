use crate::{
    decorate,
    parser::{
        decorator::Decoration,
        parser::{ColorerRegex, Parser},
    },
};

pub struct Ping;

impl Parser for Ping {
    fn regexs(&self) -> Vec<ColorerRegex> {
        vec![
            // url 1
            ColorerRegex::new(
                r"((?<=(PING\s))|(?<=(^-{3}\s)))\S+",
                decorate!(Decoration::MagentaFgBright),
                None,
            ),
            // url 2
            ColorerRegex::new(
                r"(?<=([\d+\sbytes]?\s[fF]rom\s)).+?(?=[:|\s])",
                decorate!(Decoration::CyanFgBright),
                Some(vec![(
                    r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}",
                    decorate!(Decoration::GreenFgBright),
                )]),
            ),
            // IP
            ColorerRegex::new(
                r"(?<=\s(\()).*?(?=(\)))",
                decorate!(Decoration::GreenFgBright),
                None,
            ),
            // seq
            ColorerRegex::new(
                r"(?<=(icmp_seq=))[0-9]+",
                decorate!(Decoration::YellowFgBright),
                None,
            ),
            // ttl
            ColorerRegex::new(
                r"(?<=(ttl=))[0-9]+",
                decorate!(Decoration::YellowFgBright),
                None,
            ),
            // time
            ColorerRegex::new(
                r"(?<=(time=))[0-9]+.[0-9]+",
                decorate!(Decoration::YellowFgBright),
                None,
            ),
            // host unreachable
            ColorerRegex::new(
                "Destination Host Unreachable",
                decorate!(Decoration::RedBgBright),
                None,
            ),
            // min
            ColorerRegex::new(r"min", decorate!(Decoration::BlueFgBright), None),
            // avg
            ColorerRegex::new(r"avg", decorate!(Decoration::MagentaFgBright), None),
            // max
            ColorerRegex::new(r"max", decorate!(Decoration::GreenFgBright), None),
            // mdev
            ColorerRegex::new(r"mdev|stddev", decorate!(Decoration::BlackFgBright), None),
            // min value
            ColorerRegex::new(
                r"(?<=(=\s))[0-9]+.[0-9]+",
                decorate!(Decoration::BlueFgBright),
                None,
            ),
            // avg value
            ColorerRegex::new(
                r"(?<=(=\s[0-9]+.[0-9]+)\/)[0-9]+.[0-9]+",
                decorate!(Decoration::MagentaFgBright),
                None,
            ),
            // max value
            ColorerRegex::new(
                r"(?<=(=\s[0-9]+.[0-9]+)\/[0-9]+.[0-9]+\/)[0-9]+.[0-9]+",
                decorate!(Decoration::GreenFgBright),
                None,
            ),
            // mdev value
            ColorerRegex::new(
                r"(?<=(=\s[0-9]+.[0-9]+)\/[0-9]+.[0-9]+\/[0-9]+.[0-9]+\/)[0-9]+.[0-9]+",
                decorate!(Decoration::BlackFgBright),
                None,
            ),
            // loss
            ColorerRegex::new(
                r"\S*\%",
                decorate!(Decoration::GreenFgBright),
                Some(vec![
                    (r"0%", decorate!(Decoration::GreenFgBright)),
                    (r"\d{1,2}(\.\d+)?%", decorate!(Decoration::YellowFgBright)),
                    (r"100%", decorate!(Decoration::RedBgBright)),
                ]),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        decorate,
        parser::{
            decorator::Decoration,
            parser::{init_parser, reader_handler},
        },
    };

    #[test]
    fn ping_ip() {
        let input = "PING 192.168.0.1 (192.168.0.1) 56(84) bytes of data.
64 bytes from 192.168.0.1: icmp_seq=1 ttl=64 time=1.73 ms
64 bytes from 192.168.0.1: icmp_seq=2 ttl=64 time=1.38 ms
64 bytes from 192.168.0.1: icmp_seq=3 ttl=64 time=1.40 ms

--- 192.168.0.1 ping statistics ---
3 packets transmitted, 3 received, 0% packet loss, time 5ms
rtt min/avg/max/mdev = 1.381/1.504/1.729/0.162 ms"
            .to_string();

        let correct_output =
            format!("PING {magenta}192.168.0.1{reset} ({green}192.168.0.1{reset}) 56(84) bytes of data.
64 bytes from {green}192.168.0.1{reset}: icmp_seq={yellow}1{reset} ttl={yellow}64{reset} time={yellow}1.73{reset} ms
64 bytes from {green}192.168.0.1{reset}: icmp_seq={yellow}2{reset} ttl={yellow}64{reset} time={yellow}1.38{reset} ms
64 bytes from {green}192.168.0.1{reset}: icmp_seq={yellow}3{reset} ttl={yellow}64{reset} time={yellow}1.40{reset} ms

--- {magenta}192.168.0.1{reset} ping statistics ---
3 packets transmitted, 3 received, {green}0%{reset} packet loss, time 5ms
rtt {blue}min{reset}/{magenta}avg{reset}/{green}max{reset}/{black}mdev{reset} = {blue}1.381{reset}/{magenta}1.504{reset}/{green}1.729{reset}/{black}0.162{reset} ms",
                    green = decorate!(Decoration::GreenFgBright),
                    reset = decorate!(Decoration::Default),
                    yellow = decorate!(Decoration::YellowFgBright),
                    blue = decorate!(Decoration::BlueFgBright),
                    magenta = decorate!(Decoration::MagentaFgBright),
                    black = decorate!(Decoration::BlackFgBright),
            );

        assert_eq!(
            correct_output,
            reader_handler(input, &init_parser("ping").unwrap())
        );
    }

    #[test]
    fn ping_url() {
        let input = "PING crates.io (13.226.175.95) 56(84) bytes of data.
64 bytes from server-13-226-175-95.mxp64.r.cloudfront.net (13.226.175.95): icmp_seq=1 ttl=243 time=39.0 ms
64 bytes from server-13-226-175-95.mxp64.r.cloudfront.net (13.226.175.95): icmp_seq=2 ttl=243 time=30.3 ms
64 bytes from server-13-226-175-95.mxp64.r.cloudfront.net (13.226.175.95): icmp_seq=3 ttl=243 time=35.3 ms

--- crates.io ping statistics ---
3 packets transmitted, 3 received, 0% packet loss, time 5ms
rtt min/avg/max/mdev = 30.342/34.850/38.955/3.527 ms"
            .to_string();

        let correct_output =
            format!("PING {magenta}crates.io{reset} ({green}13.226.175.95{reset}) 56(84) bytes of data.
64 bytes from {cyan}server-13-226-175-95.mxp64.r.cloudfront.net{reset} ({green}13.226.175.95{reset}): icmp_seq={yellow}1{reset} ttl={yellow}243{reset} time={yellow}39.0{reset} ms
64 bytes from {cyan}server-13-226-175-95.mxp64.r.cloudfront.net{reset} ({green}13.226.175.95{reset}): icmp_seq={yellow}2{reset} ttl={yellow}243{reset} time={yellow}30.3{reset} ms
64 bytes from {cyan}server-13-226-175-95.mxp64.r.cloudfront.net{reset} ({green}13.226.175.95{reset}): icmp_seq={yellow}3{reset} ttl={yellow}243{reset} time={yellow}35.3{reset} ms

--- {magenta}crates.io{reset} ping statistics ---
3 packets transmitted, 3 received, {green}0%{reset} packet loss, time 5ms
rtt {blue}min{reset}/{magenta}avg{reset}/{green}max{reset}/{black}mdev{reset} = {blue}30.342{reset}/{magenta}34.850{reset}/{green}38.955{reset}/{black}3.527{reset} ms",
                    green = decorate!(Decoration::GreenFgBright),
                    reset = decorate!(Decoration::Default),
                    yellow = decorate!(Decoration::YellowFgBright),
                    blue = decorate!(Decoration::BlueFgBright),
                    magenta = decorate!(Decoration::MagentaFgBright),
                    black = decorate!(Decoration::BlackFgBright),
                    cyan = decorate!(Decoration::CyanFgBright)
            );

        assert_eq!(
            correct_output,
            reader_handler(input, &init_parser("ping").unwrap())
        );
    }

    #[test]
    fn ping_host_not_reachable() {
        let input = "PING 192.168.0.2 (192.168.0.2) 56(84) bytes of data.
From 192.168.0.117 icmp_seq=1 Destination Host Unreachable
From 192.168.0.117 icmp_seq=2 Destination Host Unreachable
From 192.168.0.117 icmp_seq=3 Destination Host Unreachable

--- 192.168.0.2 ping statistics ---
3 packets transmitted, 0 received, +3 errors, 100% packet loss, time 45ms"
            .to_string();

        let correct_output =
            format!("PING {magenta}192.168.0.2{reset} ({green}192.168.0.2{reset}) 56(84) bytes of data.
From {green}192.168.0.117{reset} icmp_seq={yellow}1{reset} {red_bg}Destination Host Unreachable{reset}
From {green}192.168.0.117{reset} icmp_seq={yellow}2{reset} {red_bg}Destination Host Unreachable{reset}
From {green}192.168.0.117{reset} icmp_seq={yellow}3{reset} {red_bg}Destination Host Unreachable{reset}

--- {magenta}192.168.0.2{reset} ping statistics ---
3 packets transmitted, 0 received, +3 errors, {red_bg}100%{reset} packet loss, time 45ms",
                    green = decorate!(Decoration::GreenFgBright),
                    reset = decorate!(Decoration::Default),
                    yellow = decorate!(Decoration::YellowFgBright),
                    red_bg = decorate!(Decoration::RedBgBright),
                    magenta = decorate!(Decoration::MagentaFgBright),
            );

        assert_eq!(
            correct_output,
            reader_handler(input, &init_parser("ping").unwrap())
        )
    }

    #[test]
    fn ping_warning_percentage() {
        let input = "24 packets transmitted, 21 received, 12% packet loss, time 71ms".to_string();
        let correct_output = format!(
            "24 packets transmitted, 21 received, {yellow}12%{reset} packet loss, time 71ms",
            yellow = decorate!(Decoration::YellowFgBright),
            reset = decorate!(Decoration::Default)
        );

        assert_eq!(
            correct_output,
            reader_handler(input, &init_parser("ping").unwrap())
        );
    }

    #[test]
    fn ping_stddev() {
        let input = "round-trip min/avg/max/stddev = 23.568/27.439/29.741/2.754 ms";
        let correct_output = format!(
            "round-trip {blue}min{reset}/{magenta}avg{reset}/{green}max{reset}/{black}stddev{reset} = {blue}23.568{reset}/{magenta}27.439{reset}/{green}29.741{reset}/{black}2.754{reset} ms",
            blue = decorate!(Decoration::BlueFgBright),
            magenta = decorate!(Decoration::MagentaFgBright),
            green = decorate!(Decoration::GreenFgBright),
            black = decorate!(Decoration::BlackFgBright),
            reset = decorate!(Decoration::Default)
        );

        assert_eq!(
            correct_output,
            reader_handler(input.to_string(), &init_parser("ping").unwrap())
        );
    }
}
