# Game-in-rust
A game in rust to learn rust and some opengl

# Source
 Based on the tutorial/walkthrough http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html


# Tasks
- [x] Shots collision should also rotate
- [x] Cool down on shots
- [x] Enemy Shooting at player
- [x] Enemies collide with each other
- [x] Be able to load models
- [x] Take console input while running.
- [x] Change shaders on the fly, with notify and recompile of shaders
- [x] Load .obj models
- [x] Sword follow player
- [x] Sword rotation is correct
- [x] Sword Swing animation
- [x] Trigger animation/action on controllerInput
- [x] Sword collision, based on its location
- [x] Complex model
- [x] Player/entity/action state. Some actions has to finishe before new input is valid
- [x] Sword animation only when attacking state
- [x] render of collision boxes
- [x] third person follow cam
- [x] camera rotate around player, ie. if distance less then follow_len don't update camera pos.
- [x] return weapon anchor model-plane along with models, if existing
- [x] Use weapon achor to get normal (weapon dir) and position in animation, use to generate an action that can be used for render and hitbox
- [x] load collada with textures
- [x] Run animation
- [x] Idle animation
- [x] blender model as player model
- [x] Play run when moving and idle when not
- [x] Smooth transition between animations
- [x] load weapon and achor to weapon bone
- [x] load hitbox from blender models
- [ ] Hitboxes from bones/some bones. fx arms and legs
- [ ] Follow up/combo attack animations
- [ ] properties on animation, like when the attack is active, when a follow up can be triggered and more

# complex model
based on multiple models, fx sword follow player, or hat or weapon, legs that are fixed to fx a player and when player move sword moves to. If we swing a sword we take into account the player physics too.

# Cool stuff
distortion shader fx https://lindenreid.wordpress.com/2018/03/05/heat-distortion-shader-tutorial/#:~:text=The%20basic%20premise%20of%20the,uses%20to%20sample%20that%20texture.


# Export of models from blender
Z up and -X forward
