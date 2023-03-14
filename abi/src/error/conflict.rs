use std::{collections::HashMap, convert::Infallible, str::FromStr};

use chrono::NaiveDateTime;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum ReservationConflictInfo {
    Parsed(ReservationConflict),
    Unparsed(String),
}

#[derive(Debug, Clone)]
pub struct ReservationConflict {
    new: ReservationWindow,
    old: ReservationWindow,
}

#[derive(Debug, Clone)]
pub struct ReservationWindow {
    rid: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl FromStr for ReservationConflictInfo {
    type Err = Infallible; // 不会出错

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(conflict) = s.parse() {
            Ok(ReservationConflictInfo::Parsed(conflict))
        } else {
            Ok(ReservationConflictInfo::Unparsed(s.to_string()))
        }
    }
}

impl FromStr for ReservationConflict {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let info: Self = ParsedInfo::from_str(s).unwrap().try_into().unwrap();

        Ok(info)
    }
}

impl TryFrom<ParsedInfo> for ReservationConflict {
    type Error = String;

    fn try_from(ps_info: ParsedInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            new: ps_info.new.try_into()?,
            old: ps_info.old.try_into()?,
        })
    }
}

impl TryFrom<HashMap<String, String>> for ReservationWindow {
    type Error = String;

    fn try_from(map: HashMap<String, String>) -> Result<Self, Self::Error> {
        let timespan_str = map
            .get("timespan")
            .ok_or("1")?
            .replace('"', "")
            .replace('\\', "");
        let mut split = timespan_str.splitn(2, ',');
        // println!("split{:?}", split.next().ok_or("2")?.trim());
        let start = parsed_for_navidate(split.next().ok_or("2")?.trim());
        let end = parsed_for_navidate(split.next().ok_or("3")?.trim());
        Ok(Self {
            rid: map.get("resource_id").ok_or("6")?.to_string(),
            start,
            end,
        })
    }
}

struct ParsedInfo {
    new: HashMap<String, String>,
    old: HashMap<String, String>,
}

impl FromStr for ParsedInfo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // use regular expression to parse the string
        let re = Regex::new(r"\((?P<k1>[a-zA-Z0-9_-]+),\s*(?P<k2>[a-zA-Z0-9_-]+)\)=\((?P<v1>[a-zA-Z0-9_-]+)\s*,\s*\[(?P<v2>[^\)\]]+)").unwrap();
        let mut maps = vec![];

        for cap in re.captures_iter(s) {
            let mut map = HashMap::new();
            // map.insert(cap["k1"].to_string(), cap["v1"].to_string());
            // map.insert(cap["k2"].to_string(), cap["v2"].to_string());
            map.insert(
                cap.name("k1")
                    .expect("获取失败！ cap.name function")
                    .as_str()
                    .to_string(),
                cap.name("v1")
                    .expect("获取失败！ cap.name function")
                    .as_str()
                    .to_string(),
            );
            map.insert(
                cap.name("k2").unwrap().as_str().to_string(),
                cap.name("v2").unwrap().as_str().to_string(),
            );
            maps.push(map);
        }

        if maps.len() != 2 {
            return Err("mpas length not eq 2".into());
        }

        Ok(Self {
            new: maps[0].clone(),
            old: maps[1].clone(),
        })
    }
}

fn parsed_for_navidate(str: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S%#z").unwrap()
}

#[cfg(test)]
mod conflict_tests {
    use super::ParsedInfo;
    use crate::{error::conflict::parsed_for_navidate, ReservationConflictInfo};

    const SOURCE_STR: &str="Key (resource_id, timespan)=(ocean-view-room-713, [\"2022-12-26 22:00:00+00\",\"2022-12-30 19:00:00+00\")) conflicts with existing key (resource_id, timespan)=(ocean-view-room-713, [\"2022-12-25 22:00:00+00\",\"2022-12-28 19:00:00+00\")).";

    #[test]
    fn parsed_info_should_work() {
        let info: ParsedInfo = SOURCE_STR.parse().unwrap();
        assert_eq!(info.new["resource_id"], "ocean-view-room-713");
        assert_eq!(
            info.new["timespan"],
            "\"22022-12-26T22:00:00+00:00\",\"2022-12-30T19:00:00+00:00\""
        );
    }

    #[test]
    fn error_message_should_Parse() {
        let info: ReservationConflictInfo = SOURCE_STR.parse().unwrap();

        match info {
            ReservationConflictInfo::Parsed(conflict) => {
                assert_eq!(conflict.new.rid, "ocean-view-room-713");
                assert_eq!(
                    conflict.new.start,
                    parsed_for_navidate("2022-12-26 22:00:00+00")
                );
                assert_eq!(
                    conflict.new.end,
                    parsed_for_navidate("2022-12-30 19:00:00+00")
                );
                assert_eq!(conflict.old.rid, "ocean-view-room-713");
                assert_eq!(
                    conflict.old.start,
                    parsed_for_navidate("2022-12-25 22:00:00+00")
                );
                assert_eq!(
                    conflict.old.end,
                    parsed_for_navidate("2022-12-28 19:00:00+00")
                );
            }
            ReservationConflictInfo::Unparsed(_) => panic!("Unparsed; 无需转换"),
        }
    }
}
