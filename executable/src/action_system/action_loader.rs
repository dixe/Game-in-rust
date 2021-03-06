use nalgebra as na;

use crate::resources::{self, Resources} ;

use crate::action_system::*;


#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad { name: String, inner: resources::Error },
    #[fail(display = "Version Error")]
    VersionError,
    #[fail(display = "Xml error")]
    Xml(roxmltree::Error),
    #[fail(display = "Missing attribute {}", attrib)]
    MissingAttrib { attrib: String },
    #[fail(display = "No curve found")]
    NoCurve,
    #[fail(display = "MIssing control point {}", p)]
    MissingControlPoint { p : String}
}




impl From<roxmltree::Error> for Error {
    fn from(other: roxmltree::Error) -> Self {
        Error::Xml(other)
    }
}



pub fn load_player_actions(res: &Resources) -> Result<ActionsImpl, Error> {

    println!("Loading swing");
    let swing_name = "actions/swing.xml";
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
    parse_xml(input)
}

fn parse_xml(input: &str) -> Result<Vec<Part>, Error> {

    let doc = roxmltree::Document::parse(input)?;
    let root = &doc.root_element();

    let version = get_attrib::<i32>(root, "version")?;


    match version {
        1 => parse_xml_v1(&doc.root_element()),
        _ => Err(Error::VersionError)
    }

}




fn get_attrib<T>(node: &roxmltree::Node, name: &str) -> Result<T, Error>
where T: std::str::FromStr {
    node.attribute(name).ok_or(Error::MissingAttrib {attrib: name.to_string() } ).map(|s| s.to_string())
        .and_then(|v: String| v.parse::<T>().map_err(|_e| Error::VersionError))
}

fn parse_xml_v1(node: &roxmltree::Node) -> Result<Vec<Part>, Error> {
    parse_xml_parts(node)
}





fn parse_xml_parts(node: &roxmltree::Node) -> Result<Vec<Part>, Error> {
    let mut parts = Vec::new();

    let _next = node.descendants().find(|n| n.has_tag_name("part")) ;

    for next in node.descendants().filter(|n| n.has_tag_name("part")) {
        let part = parse_xml_part(&next)?;
        parts.push(part);
    }

    Ok(parts)
}

fn parse_xml_part(node: &roxmltree::Node) -> Result<Part, Error> {

    // parse a part

    let cubic = node.descendants().find(|n| n.has_tag_name("cubic"));

    let positions_curve: Option<action_system::Curve>;

    positions_curve = cubic.and_then(|c| {
        match parse_xml_cubic(&c) {
            Ok(cub) => Some(cub),
            _ => None
        }

    });


    if positions_curve.is_none() {
        return Err(Error::NoCurve);
    }


    let start = get_attrib::<f32>(node, "start")?;
    let end = get_attrib::<f32>(node, "end")?;

    let positions: action_system::Curve = positions_curve.unwrap();


    //TODO load of normals into a curve list positions
    let normals = Curve::Linear(na::Vector3::new(0.0, 0.0, 1.0), na::Vector3::new(0.0, 0.0, 1.0));


    Ok(Part{
        positions,
        normals,
        start,
        end})
}



fn parse_xml_cubic(node: &roxmltree::Node) -> Result<Curve, Error> {
    let p0 = node.descendants().find(|n| n.has_tag_name("p0")).ok_or(Error::MissingControlPoint {p: "p0".to_string()}).and_then(|p| parse_xml_point(&p))?;
    let p1 = node.descendants().find(|n| n.has_tag_name("p1")).ok_or(Error::MissingControlPoint {p: "p1".to_string()}).and_then(|p| parse_xml_point(&p))?;
    let p2 = node.descendants().find(|n| n.has_tag_name("p2")).ok_or(Error::MissingControlPoint {p: "p2".to_string()}).and_then(|p| parse_xml_point(&p))?;

    Ok(Curve::Cubic(p0, p1, p2))
}

fn parse_xml_point(node: &roxmltree::Node) -> Result<na::Vector3::<f32>, Error> {

    let x = get_attrib::<f32>(node, "x")?;
    let y = get_attrib::<f32>(node, "y")?;
    let z = get_attrib::<f32>(node, "z")?;

    Ok(na::Vector3::new(x,y,z))

}

#[cfg(test)]
mod tests {



}
