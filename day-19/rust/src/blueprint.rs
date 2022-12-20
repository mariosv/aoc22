#[derive(Debug)]
pub struct Blueprint {
    pub ore: u32,
    pub clay: u32,
    pub obsidian: ObsidianRobotRequirements,
    pub geode: GeodeRobotRequirements
}

#[derive(Debug)]
pub struct ObsidianRobotRequirements {
    pub ore: u32,
    pub clay: u32
}

#[derive(Debug)]
pub struct GeodeRobotRequirements {
    pub ore: u32,
    pub obsidian: u32
}

pub fn parse(line: &String) -> Blueprint {
    let s: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();

    let ore: u32 = s[6].parse().unwrap();
    let clay: u32 = s[12].parse().unwrap();
    let obsidian = ObsidianRobotRequirements {
        ore: s[18].parse().unwrap(),
        clay: s[21].parse().unwrap()
    };
    let geode = GeodeRobotRequirements {
        ore: s[27].parse().unwrap(),
        obsidian: s[30].parse().unwrap()
    };

    Blueprint { ore, clay, obsidian, geode }
}
