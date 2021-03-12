use crate::game;
use crate::physics;
use crate::shot;
use crate::entity;

pub fn update_game_state(ctx: &mut game::Context, collisions: &physics::Collisions) {
    // maybe not have this as member function, since it not realy here it should be

    update_player_shoot(ctx);


    for c in &collisions.enemies_hit {
        let mut enemy_hp = match ctx.ecs.get_health(c.entity_id) {
            Some(e) => *e,
            _ => continue
        };

        let dead = enemy_hp.damage(1.0);

        if dead {
            ctx.ecs.remove_entity(c.entity_id);
        }
        else {
            ctx.ecs.set_health(c.entity_id, enemy_hp);
        }

        ctx.ecs.remove_entity(c.projectile_id);
    }
}


fn update_player_shoot(ctx: &mut game::Context) {
    let delta = ctx.get_delta_millis();
    for p in &mut ctx.player_projectiles {
        p.update(delta);

        if p.expired {
            ctx.ecs.remove_entity(p.entity_id);
        }
    }

    ctx.player_projectiles.retain(|p| !p.expired);


    let shoot_dir = ctx.controls.shoot_dir;
    let player_id = ctx.player_id;

    match shoot_dir {
        Some(dir) =>
        {
            add_projectile(ctx,  dir, player_id);
        },
        _ => {}
    };

}


fn add_projectile(ctx: &mut game::Context, shoot_dir: na::Vector3::<f32>, entity_id: usize) {

    let shooter = match ctx.ecs.get_shooter(entity_id) {
        Some(s) => s,
        None => return,
    };

    let mut entity_pos = match ctx.ecs.get_physics(entity_id) {
        Some(s) => s.pos,
        None => return,
    };


    entity_pos.z += 0.3; // get shoot heght from shooter


    let rotation_cos = na::Vector3::new(1.0, 0.0, 0.0).dot(&shoot_dir.normalize());
    let rotation_sin_vec = na::Vector3::new(1.0, 0.0, 0.0).cross(&shoot_dir.normalize());
    let rotation_sin = rotation_sin_vec.z.signum() * rotation_sin_vec.magnitude();

    let speed = 30.0;

    let vel = shoot_dir.normalize() * speed;

    let id = ctx.ecs.add_entity();

    let physics = entity::Physics {
        entity_id: id,
        pos: entity_pos,
        velocity: vel,
        max_speed: speed,
        rotation_sin,
        rotation_cos,
        acceleration: speed,
        //TODO removee from phyiscs, and // get model id by entity_id
        model_id: ctx.player_projectile_model_id,
    };


    ctx.ecs.set_physics(id, physics);
    let shot = shot::Shot::new(id, 300);

    ctx.player_projectiles.push(shot);

}
