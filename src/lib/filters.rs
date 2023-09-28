/**
 * filters.rs
 * This file handles the filtering of proxies.
 * It also morphs them (i.e. assumes a port if there is none)
 */
use regex::Regex;

// Wether or not the filtered proxy is accepted and if not, if it is changed
struct FilterAction {
    accepted: bool,
    new_value: Option<String>,
}

// Filters given list of raw proxies as Vec.
pub fn filter_all(proxies: Vec<String>) -> Vec<String> {
    let mut filtered_proxies: Vec<String> = Vec::new();
    let mut splitted_proxies: Vec<String> = Vec::new();

    // This part removes newlines from the proxy list
    for proxy in proxies {
        if proxy.is_empty() || proxy == "" {
            // failsafe
            continue;
        }
        if !proxy.contains("\n") && !proxy.contains("\r") {
            splitted_proxies.push(proxy);
            continue;
        }
        let to_push = proxy
            .as_str()
            .split(|c| c == '\r' || c == '\n')
            .collect::<Vec<&str>>();
        for proxy in to_push {
            splitted_proxies.push(proxy.to_string())
        }
    }

    for proxy in splitted_proxies {
        let filtered = filter_proxy(proxy.clone());

        if filtered.accepted {
            filtered_proxies.push(proxy.clone());
        } else if let Some(new_value) = filtered.new_value {
            filtered_proxies.push(new_value.clone());
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
        r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$",
        relevant_proxy.clone(),
    ) {
        /* simplest proxy format (no port) (ipv4) */
        // return while assuming port
        return FilterAction {
            accepted: false,
            new_value: Some(format!("{}:80", relevant_proxy.clone())),
        };
    } else if test(
        r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}$",
        relevant_proxy.clone(),
    ) {
        /* simple proxy format (w/ port) (ipv4)
        xxx.xxx.xxx:xxxxx */
        return FilterAction {
            accepted: true,
            new_value: None,
        };
    } else if test(
        r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}@SOCKS[4,5]$",
        relevant_proxy.clone(),
    ) {
        /* socks proxy format (w/ port) (ipv4)
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
