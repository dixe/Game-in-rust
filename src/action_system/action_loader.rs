use nalgebra as na;

use std::iter::Peekable;
use std::str::Chars;


use crate::resources::{self, Resources} ;
use crate::entity;
use crate::action_system::*;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad { name: String, inner: resources::Error },

    #[fail(display = "Can not parse the point: {}, because {}", input, reason)]
    ParsePointFailed { input: String, reason: String },

    #[fail(display = "Can not parse cubic: {}, because {}", input, reason)]
    ParseCubicFailed { input: String, reason: String },

    #[fail(display = "Version Error got: {}", input)]
    VersionError { input: String },

    #[fail(display = "Expected a '{}' got: '{}'", input, got)]
    ExpectedError { input: char, got: char },

}


pub fn load_player_actions(res: &Resources) -> Result<ActionsImpl, Error> {

    let swing_name = "actions/swing.act";
    let swing_data = res.load_string(swing_name)
        .map_err(|e| Error::ResourceLoad {
            name: swing_name.into(),
            inner: e
        })?;

    println!("{}", swing_data);


    let parts = parse(&swing_data)?;

    let swing = BezierAction{ parts };
    let idle = FuncAction { update_fn: idle_bob_z};

    Ok(ActionsImpl {
        swing,
        idle
    })




}


pub struct ActionParser<'a> {
    chars: Peekable<Chars<'a>>,
    parts_q: Vec<String>
}

impl<'a> ActionParser<'a> {

    pub fn new(data: &str) -> ActionParser {
        ActionParser {
            chars: data.chars().peekable(),
            parts_q: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Part>, Error> {
        let mut parts = Vec::new();

        while self.chars.peek().is_some() {
            self.consume_while(char::is_whitespace);
            self.expect_string("version: 1")?;
            self.consume_while(char::is_whitespace);
            self.expect_string("part")?;

            self.parse_part()?;

            self.consume_while(char::is_whitespace);
            return Ok(parts);

        }

        Ok(parts)
    }

    fn parse_part(&mut self) -> Result<Part, Error> {


        while self.chars.peek().is_some() {
            // parse curve:
            // either
            // if starts with c: then parse cubic
            // if starts with l: then parse linear

            // parse start and end

        }


        let curve =  Curve::Linear(na::Vector3::identity(),na::Vector3::identity());

        let part = Part {
            curve,
            start: 0.0,
            end: 1.0
        };

        Ok(part)
    }


    fn one_of<F, T> (&mut self, func: Vec<F>) -> Result<T, Error>
    where F: Fn(ActionParser) -> Result<T,Error> {



    }

    fn try_parse<F, T> (&mut self, func: F) -> Result<T, Error>
    where F: Fn(ActionParser) -> Result<T,Error> {





    }


    fn expect_string(&mut self, expected: &str) -> Result<String,Error> {

        let mut parsed = String::new();
        let peek = self.chars.peek();

        for next in expected.chars() {
            if !self.chars.next().map_or(false, |c| c == next) {
                return Err(Error::ExpectedError { input: self.chars.next().unwrap(), got: next });
            }

            parsed.push(next);
        }

        Ok(parsed)
    }




    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }

        result
    }

    fn rest_of_input(&mut self) -> String {
        let mut result = String::new();
        while self.chars.peek().is_some() {
            result.push(self.chars.next().unwrap());
        }

        result

    }



}

fn parse(input: &str) -> Result<Vec<Part>, Error> {
    let lines = input.lines();

    let mut vec_lines:Vec<&str> = input.lines().collect();

    match vec_lines[0].trim() {
        "version: 1" => parse_v1(lines.skip(1).collect()),
        _ =>  Err(Error::VersionError { input: vec_lines[0].to_string() })
    }

}


fn parse_v1(lines: Vec<&str>) -> Result<Vec<Part>, Error> {

    let mut parts = Vec::<Part>::new();


    let mut height: i32 = 0;
    let mut width: i32 = 0;
    for line in lines.iter() {
        let curve = parse_cubic(line)?;

        let part = Part {
            curve,
            start: 0.0,
            end: 1.0
        };
        parts.push(part);
    }


    Ok(parts)
}

fn parse_cubic(string_data: &str) -> Result<Curve, Error> {
    let data = string_data.split('|').collect::<Vec<_>>();
    if data.len() != 3 {
        return Err(Error::ParseCubicFailed { input: string_data.to_string(), reason: "length was not 3".to_string()})
    }

    let p0 = parse_point(data[0])?;
    let p1 = parse_point(data[1])?;
    let p2 = parse_point(data[2])?;

    Ok(Curve::Cubic(p0, p1, p2))


}

fn parse_point(string_data: &str) -> Result<na::Vector3::<f32>, Error> {

    let data = string_data.split(',').collect::<Vec<&str>>();

    if data.len() != 3 {
        return Err(Error::ParsePointFailed { input: string_data.to_string(), reason: "length was not 3".to_string()})
    }

    let x = data[0].trim().parse::<f32>().map_err(|_| Error::ParsePointFailed { input: string_data.to_string(), reason: "Error parsing x".to_string()})?;
    let y = data[1].trim().parse::<f32>().map_err(|_| Error::ParsePointFailed { input: string_data.to_string(), reason: "Error parsing y".to_string()})?;
    let z = data[2].trim().parse::<f32>().map_err(|_| Error::ParsePointFailed { input: string_data.to_string(), reason: "Error parsing z".to_string()})?;


    Ok(na::Vector3::new(x,y,z))
}


#[cfg(test)]
mod tests {

    use crate::action_system::action_loader::*;



    #[test]
    fn parser_test_01() {

        let input = "version: 1
part
c: 0,0,0 | 3,0,0 | 0,0,0
";


        let mut parser = ActionParser::new(input);

        let res = parser.parse();

        match res {
            Ok(r) => {
                assert!(r.len() == 1);
            },
            Err(err) => {
                println!("{:#?}", err);
                assert!(false);
            }
        };

    }

}
