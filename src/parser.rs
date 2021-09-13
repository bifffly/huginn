use crate::*;

enum List {
    BLIST,
    NLIST,
    NONE
}

fn get_arg(text: &str) -> &str {
    let split_right = text.split("@head(").collect::<Vec<&str>>()[1];
    let split_left = split_right.split(")").collect::<Vec<&str>>()[0];
    return split_left;
}

pub fn odin_to_components(text: String) -> Vec<HuginnComponent> {
    let mut label_strings = vec![];
    let mut list_status = List::NONE;
    let mut nlist_index = 1;

    let mut lines = text.split("\n").collect::<Vec<&str>>();
    lines.retain(|line| !line.is_empty());
    for mut line in lines {
        let mut blist_bullet = "  • ".to_string();
        let mut nlist_bullet = format!("  {}. ", nlist_index).to_string();
        match list_status {
            List::NONE => {},
            _ => {
                let split = line.split("*").collect::<Vec<&str>>()[0];
                line = split.trim_start();
            }
        }
        if line.starts_with("@head") {
            let text = "<span size=\"xx-large\"><b>".to_owned() + get_arg(line).trim_end() + "</b></span>";
            label_strings.push(text);
        }
        else if line.starts_with("@hhead") {
            let text = "<span size=\"x-large\"><b>".to_owned() + get_arg(line).trim_end() + "</b></span>";
            label_strings.push(text);
        }
        else if line.starts_with("@hhhead") {
            let text = "<span size=\"large\"><b>".to_owned() + get_arg(line).trim_end() + "</b></span>";
            label_strings.push(text);
        }
        else if line.starts_with("@nlist") {
            match list_status {
                List::NLIST => {
                    nlist_index = 1;
                    list_status = List::NONE;
                },
                _ => list_status = List::NLIST
            }
        }
        else if line.starts_with("@blist") {
            list_status = match list_status {
                List::BLIST => List::NONE,
                _ => List::BLIST
            }
        }
        else {
            match list_status {
                List::BLIST => {
                    blist_bullet.push_str(line);
                    line = &blist_bullet;
                },
                List::NLIST => {
                    nlist_bullet.push_str(line);
                    line = &nlist_bullet;
                    nlist_index += 1;
                },
                _ => {}
            }
            label_strings.push(line.trim_end().to_string());
        }
    }
    return label_strings.iter().map(HuginnComponent::new).collect::<Vec<HuginnComponent>>();
}

