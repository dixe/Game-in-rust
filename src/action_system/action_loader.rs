use nalgebra as na;

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

}


pub fn load_player_actions(res: &Resources) -> Result<ActionsImpl, Error> {

    let swing_name = "actions/swing.act";
    let swing_data = res.load_string(swing_name)
        .map_err(|e| Error::ResourceLoad {
            name: swing_name.into(),
            inner: e
        })?;


    let parts = parse(&swing_data)?;

    let swing = BezierAction{ parts };
    let idle = FuncAction { update_fn: idle_bob_z};

    Ok(ActionsImpl {
        swing,
        idle
    })




}

fn parse(input: &str) -> Result<Vec<Part>, Error> {
    let lines = input.lines();

    // TODO CHECK VERSION INFO

    parse_v1(lines)
}


fn parse_v1(lines: std::str::Lines) -> Result<Vec<Part>, Error> {

    let mut parts = Vec::<Part>::new();


    let mut height: i32 = 0;
    let mut width: i32 = 0;
    for (_, line) in lines.enumerate() {
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
