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

            ctx.add_player_projectile(dir);
        }
        _ => {}
    }
}
