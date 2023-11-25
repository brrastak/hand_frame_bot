
#[derive(PartialEq)]
pub enum FrameStyle {
    LightCenter,
    DarkCenter
}

struct Hand<'a> {
    up: &'a str,
    down: &'a str,
    right: &'a str,
    left: &'a str,
}

const BRIGHTNESS_STEPS: usize = 5;

pub fn max_layers_number() -> usize {
    BRIGHTNESS_STEPS
}


pub fn new(central_symbol: &str, layers: usize, style: FrameStyle) -> Option<Vec<String>> {

    let mut res = Vec::new();
    res.push(String::from(central_symbol));

    let hands = get_hand_set(style);
    let hands = hands.get(..layers)?;

    let mut current_layer = 0;

    for hand in hands {

        current_layer += 1;

        for line in &mut res {

            line.insert_str(0, hand.right);
            line.push_str(hand.left);
        }
        res.insert(0, hand.down.repeat(get_num_of_cols(current_layer)));
        res.push(hand.up.repeat(get_num_of_cols(current_layer)));
    }

    Some(res)
}


fn get_hand_set(style: FrameStyle) -> Vec<Hand<'static>> {

    if style == FrameStyle::LightCenter {

        vec![
            Hand{up:"👆🏻", down:"👇🏻", right:"👉🏻", left:"👈🏻"},
            Hand{up:"👆🏼", down:"👇🏼", right:"👉🏼", left:"👈🏼"},
            Hand{up:"👆🏽", down:"👇🏽", right:"👉🏽", left:"👈🏽"},
            Hand{up:"👆🏾", down:"👇🏾", right:"👉🏾", left:"👈🏾"},
            Hand{up:"👆🏿", down:"👇🏿", right:"👉🏿", left:"👈🏿"},
        ]
    }
    else {
        vec![
            Hand{up:"👆🏿", down:"👇🏿", right:"👉🏿", left:"👈🏿"},
            Hand{up:"👆🏾", down:"👇🏾", right:"👉🏾", left:"👈🏾"},
            Hand{up:"👆🏽", down:"👇🏽", right:"👉🏽", left:"👈🏽"},
            Hand{up:"👆🏼", down:"👇🏼", right:"👉🏼", left:"👈🏼"},
            Hand{up:"👆🏻", down:"👇🏻", right:"👉🏻", left:"👈🏻"},
        ]
    }
}

fn get_num_of_cols(layer: usize) -> usize {
    layer * 2 + 1
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    #[should_panic]
    fn too_much_layers() {

        new("❤️", 6, FrameStyle::LightCenter).unwrap();
    }

    #[test]
    fn one_layer_light() {

        let test = new(
            "❤️", 
            1, 
            FrameStyle::LightCenter)
            .unwrap();
        let expected = vec![
            "👇🏻👇🏻👇🏻",
            "👉🏻❤️👈🏻",
            "👆🏻👆🏻👆🏻"
        ];
        assert_eq!(test, expected);
    }

    #[test]
    fn five_layers_dark() {

        let test = new(
            "❤️", 
            5, 
            FrameStyle::DarkCenter)
            .unwrap();
        let expected = vec![
            "👇🏻👇🏻👇🏻👇🏻👇🏻👇🏻👇🏻👇🏻👇🏻👇🏻👇🏻",
            "👉🏻👇🏼👇🏼👇🏼👇🏼👇🏼👇🏼👇🏼👇🏼👇🏼👈🏻",
            "👉🏻👉🏼👇🏽👇🏽👇🏽👇🏽👇🏽👇🏽👇🏽👈🏼👈🏻",
            "👉🏻👉🏼👉🏽👇🏾👇🏾👇🏾👇🏾👇🏾👈🏽👈🏼👈🏻",
            "👉🏻👉🏼👉🏽👉🏾👇🏿👇🏿👇🏿👈🏾👈🏽👈🏼👈🏻",
            "👉🏻👉🏼👉🏽👉🏾👉🏿❤️👈🏿👈🏾👈🏽👈🏼👈🏻",
            "👉🏻👉🏼👉🏽👉🏾👆🏿👆🏿👆🏿👈🏾👈🏽👈🏼👈🏻",
            "👉🏻👉🏼👉🏽👆🏾👆🏾👆🏾👆🏾👆🏾👈🏽👈🏼👈🏻",
            "👉🏻👉🏼👆🏽👆🏽👆🏽👆🏽👆🏽👆🏽👆🏽👈🏼👈🏻",
            "👉🏻👆🏼👆🏼👆🏼👆🏼👆🏼👆🏼👆🏼👆🏼👆🏼👈🏻",
            "👆🏻👆🏻👆🏻👆🏻👆🏻👆🏻👆🏻👆🏻👆🏻👆🏻👆🏻",
        ];
        assert_eq!(test, expected);
    }

    #[test]
    #[ignore]
    fn correct_num_of_cols() {

        assert_eq!(get_num_of_cols(1), 3);
        assert_eq!(get_num_of_cols(5), 11);
    }

    #[test]
    #[ignore]
    fn correct_color() {

        let vec = get_hand_set(FrameStyle::LightCenter);
        let hand = vec[0].up;
        assert_eq!(hand, "👆🏻");

        let vec = get_hand_set(FrameStyle::DarkCenter);
        let hand = vec[0].up;
        assert_eq!(hand, "👆🏿");
    }
    
}

