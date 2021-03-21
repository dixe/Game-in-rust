use crate::game;
use crate::entity;



pub fn add_projectile(projectilies: &mut std::collections::HashSet<usize>,
                      ecs: &mut entity::EntityComponentSystem,
                      shooter_ref: &entity::Shooter,
                      shoot_dir: na::Vector3::<f32>,
                      entity_id: usize,
                      model_id: usize) {

    let mut shooter = *shooter_ref;
    if !shooter.can_shoot() {
        return;
    }

    let mut entity_pos = match ecs.get_physics(entity_id) {
        Some(s) => s.pos,
        None => return,
    };

    entity_pos.z += 0.3; // get shoot heght from shooter

    let rotation = game::get_rotation(&shoot_dir);

    let id = ecs.add_entity();
    let vel = shoot_dir.normalize() * shooter.speed;


    let mut physics = entity::Physics::new(id,  model_id);

    physics.pos = entity_pos;
    physics.velocity = vel;
    physics.max_speed = shooter.speed;
    physics.rotation.z = f32::atan2(rotation.sin, rotation.cos);
    physics.inverse_mass = 1.0/10.0;
    physics.scale = 0.5;
    ecs.set_physics(id, physics);


    let shot = entity::Shot::new(id, (shooter.speed * shooter.distance) / 1000.0);

    ecs.set_shot(id, shot);
    projectilies.insert(id);

    // update shooter component
    shooter.shoot();
    ecs.set_shooter(entity_id, shooter);


}
