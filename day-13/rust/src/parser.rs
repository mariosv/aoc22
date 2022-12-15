use crate::packet::Packet;

pub fn parse(s: &String) -> Result <Packet, String> {
    let mut state = State { cursor: 0,
                            buf: s.chars().collect::<Vec<_>>() };
    let list = list(&mut state)?;
    Ok(list)
}

struct State {
    cursor: usize,
    buf: Vec<char>
}

fn number(state: &mut State) -> Result<u32, String> {
    let mut tmp = Vec::<char>::new();
    loop {
        match state.buf.get(state.cursor) {
            Some(c) => {
                if c == &']' || c == &',' {
                    break;
                } else if c.is_numeric() {
                    tmp.push(*c);
                } else {
                    return Err(format!("Unexpected character: {}", c));
                }
            },
            None => break
        }
        state.cursor += 1;
    }
    if tmp.is_empty() {
        return Err("Failed to parse number".to_string())
    }
    let s: String = tmp.into_iter().collect();
    Ok(s.parse::<u32>().unwrap())
}

fn fixed_char(state: &mut State, c: char) -> Result<(), String> {
    match state.buf.get(state.cursor) {
        Some(t) => {
            if t != &c {
                return Err(format!("Expected '{}', got: '{}'", c, t))
            }
        },
        None => { return Err(format!("Failed to parse char: {}", c)); }
    }
    state.cursor += 1;
    Ok(())
}

fn list(state: &mut State) -> Result<Packet, String> {
    fixed_char(state, '[')?;
    let mut data = Vec::<Packet>::new();
    loop {
        match state.buf.get(state.cursor) {
            Some(c) => {
                if c.is_numeric() {
                    let num = number(state)?;
                    data.push(Packet::Num(num));
                } else if c == &',' {
                    state.cursor += 1;
                } else if c == &'[' {
                    let l = list(state)?;
                    data.push(l);
                } else {
                    fixed_char(state, ']')?;
                    state.cursor += 1;
                    break;
                }
            },
            None => break
        }
    }
    Ok(Packet::List(data))
}
