use crate::game;
use crate::physics;
use crate::shot;
use crate::entity;



pub fn update_game_state(ctx: &mut game::Context, collisions: &physics::Collisions) {

    let delta = ctx.get_delta_millis();

    // Update shooters, projectiles and othertime based stuff
    update_projectiles(ctx, delta);
    update_shooters(ctx, delta);


    update_player_shoot(ctx);


    // PLAYER MOVEMENT

    let mut player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(e) => *e,
        None => return, // Dead player, don't care about ai update
    };

    game::update_velocity_and_rotation(&mut player, ctx.controls.movement_dir);

    ctx.ecs.set_physics(ctx.player_id, player);



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


fn update_shooters(ctx: &mut game::Context, delta: i32) {

    for shooter in ctx.ecs.shooter.values_mut() {
        shooter.update(delta);
    }
}

fn update_projectiles(ctx: &mut game::Context, delta: i32) {
    for p in &mut ctx.player_projectiles {
        p.update(delta);

        if p.expired {
            ctx.ecs.remove_entity(p.entity_id);
        }
    }

    // enemies shot when needed
}


fn update_player_shoot(ctx: &mut game::Context) {

    let player_id = ctx.player_id;
    ctx.player_projectiles.retain(|p| !p.expired);

    let shoot_dir = ctx.controls.shoot_dir;

    match shoot_dir {
        Some(dir) =>
        {
            add_projectile(ctx,  dir, player_id);

        },
        _ => {}
    };

}


fn add_projectile(ctx: &mut game::Context, shoot_dir: na::Vector3::<f32>, entity_id: usize) {

    let mut shooter = match ctx.ecs.get_shooter(entity_id) {
        Some(s) => *s,
        None => return,
    };

    if !shooter.can_shoot() {
        return;
    }

    let mut entity_pos = match ctx.ecs.get_physics(entity_id) {
        Some(s) => s.pos,
        None => return,
    };

    entity_pos.z += 0.3; // get shoot heght from shooter


    let rotation = game::get_rotation(&shoot_dir);


    let id = ctx.ecs.add_entity();
    let vel = shoot_dir.normalize() * shooter.speed;


    let physics = entity::Physics {
        entity_id: id,
        pos: entity_pos,
        velocity: vel,
        max_speed: shooter.speed,
        rotation_sin: rotation.sin,
        rotation_cos: rotation.cos,
        acceleration: shooter.speed,
        //TODO removee from phyiscs, and // get model id by entity_id
        model_id: ctx.player_projectile_model_id,
    };




    ctx.ecs.set_physics(id, physics);


    let shot = shot::Shot::new(id, (shooter.speed * shooter.distance) as i32);

    ctx.player_projectiles.push(shot);

    // update shooter component
    shooter.shoot();
    ctx.ecs.set_shooter(entity_id, shooter);


}
