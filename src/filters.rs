use regex::Regex;

// Wether or not the filtered proxy is accepted and if not, if it is changed
struct FilterAction {
    accepted: bool,
    new_value: Option<String>,
}

// Filters given list of raw proxies as Vec.
pub fn filter_all(proxies: Vec<String>, debug: bool) -> Vec<String> {
    let mut filtered_proxies: Vec<String> = Vec::new();
    let mut splitted_proxies: Vec<String> = Vec::new();

    // This part removes newlines from the proxy list
    for proxy in proxies {
        let to_push = proxy
            .as_str()
            .split(|c| c == '\r' || c == '\n')
            .collect::<Vec<&str>>();
        for proxy in to_push {
            if !proxy.is_empty() || proxy != "" { // failsafe
                splitted_proxies.push(proxy.to_string())
            }
        }
    }

    for proxy in splitted_proxies {
        let filtered = filter_proxy(proxy.clone());

        if filtered.accepted {
            filtered_proxies.push(proxy.clone());
            if debug {
                println!("Accepted: {}", proxy);
            }
        } else {
            if let Some(new_value) = filtered.new_value {
                filtered_proxies.push(new_value.clone());
                if debug {
                    println!("Morphed: {}", new_value)
                };
            } else if debug {
                println!("Rejected: {}", proxy);
            }
        }
    }
    return filtered_proxies;
}

// possible actions to take on a proxy
enum MorphAction {
    IpPortSocks45,
}

fn morph(proxy: String, action: MorphAction) -> String {
    match action {
        MorphAction::IpPortSocks45 => {
            let ip_port_socks = proxy.split_whitespace().collect::<Vec<&str>>();
            return ip_port_socks[0]
                .to_string()
                .split("@")
                .collect::<Vec<&str>>()[0]
                .to_string();
        }
    }
}

// Filters a single proxy string
fn filter_proxy(proxy: String) -> FilterAction {
    let relevant_proxy = proxy.split_whitespace().collect::<Vec<&str>>()[0].to_string();

    fn test(reg: &str, proxy: String) -> bool {
        Regex::new(reg).unwrap().is_match(proxy.as_str())
    }

    if test(
        r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}$",
        relevant_proxy.clone(),
    ) {
        /* simplest proxy format
        xxx.xxx.xxx:xxxxx */
        return FilterAction {
            accepted: true,
            new_value: None,
        };
    } else if test(
        r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}@SOCKS[4,5]$",
        relevant_proxy.clone(),
    ) {
        /* socks proxy format
        xxx.xxx.xxx:xxxxx@SOCKSx */
        return FilterAction {
            accepted: false,
            new_value: Some(morph(relevant_proxy.clone(), MorphAction::IpPortSocks45)),
        };
    }
    FilterAction {
        accepted: false,
        new_value: None,
    }
}
