use Cheat;

pub struct Speedhack;

impl Cheat for Speedhack {
	fn get_name<'a>(&'a self) -> &'a str {
		"Speedhack"
	}
	fn process_usercmd(&mut self, &mut sdk::CUserCmd) {
		// FIXME: hook input, see which way we want to go
		// atm, we can't move backwards which is annoying,
		// and moving sideways gets regular speed
		if (cmd.buttons & (1 << 0)) == 0 && (cmd.forwardmove > 0.1f32 || cmd.forwardmove < -0.001f32) {
		// speedhack time
		let x = cmd.forwardmove;
		cmd.forwardmove = -999f32; // the server will cap this at our actual max. movement speed
		
		cmd.viewangles.pitch = 89f32;
		cmd.viewangles.yaw = ((cmd.viewangles.yaw + 180f32) % 360f32); // flip us around
		cmd.viewangles.roll= 49f32; // capped at 50 by server
	}
}