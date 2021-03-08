use crate::shot;
use crate::game;
use crate::physics;


pub fn update_game_state(ctx: &mut game::Context, collisions: &physics::Collisions) {
    // maybe not have this as member function, since it not realy here it should be

    update_player_shoot(ctx);


    for c in &collisions.enemies_hit {
        let mut enemy_hp = match ctx.ecs.get_health(c.entity_id) {
            Some(e) => e,
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


    match ctx.controls.shoot_dir {
        Some(dir) =>
        {
            //todo check cooldown/shooting speed

            // spawn projectile with dir

            let mut player_pos = match ctx.ecs.get_physics(ctx.player_id) {
                Some(p) => p.pos,
                _ => return // Can we shoot when dead, and should all exit. Maybe just update shooting in own function
            };

            player_pos.z += 0.5;

            let speed = 30.0;

            let p_id = ctx.ecs.add_entity_with_vel(ctx.player_projectile_model_id, player_pos, dir * speed);
            let shot = shot::Shot::new(p_id, 300);
            ctx.player_projectiles.push(shot);
        }
        _ => {}
    }
}
