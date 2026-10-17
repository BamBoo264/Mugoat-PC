//! Commands usable before the large game data has been copied.
use anyhow::{Context,Result};
pub fn command(args:&[String])->Result<bool>{
    match args.get(1).map(String::as_str){
        Some("--help")=>{println!("Mugoat Windows experimental\n--init: create portable user-data/config.json\n--check-game: validate engine/data/shader paths\n--gpu-check: request a real GPU device without loading the game\n--import <game.zip>: import compatible game files\nMUGOAT_GAME_DIR: external game folder (no copy needed)\nMUGOAT_GPU: vulkan or dx12 (default: both)\nMUGOAT_HOME: settings/save/cache directory\nNo arguments: launch game. Set server, room, room_pass, player_name and multiplayer in config.json. Cross-play requires the same server/protocol and compatible game data.");}
        Some("--init")=>{
            let dir=crate::config::support_dir();std::fs::create_dir_all(&dir)?;
            let p=dir.join("config.json");
            if !p.exists(){std::fs::write(&p,serde_json::to_vec_pretty(&serde_json::json!({
                "scale":0.5,"resolution":"1280x720","fullscreen":false,"fps_limit":60,"start_mode":2,
                "multiplayer":false,"host":false,"server":"","room":"default","room_pass":"","player_name":"PC Player","pvp":false,
                "texture_quality":0,"shadow_quality":0,"shadow_distance":0.0,"reflection_quality":0,"grass_quality":0,"water_quality":0,"particle_quality":0,"postfx":0,"city_density":0.0,"ped_density":0.0,"distance_scaling":0.0,"extra_args":["-shaderQuality=1"]
            }))?)?;}println!("Settings: {}\nGame folder: {}",p.display(),crate::config::game_dir().display());
        }
        Some("--check-game")=>{let s=crate::config::setup()?;crate::config::validate_game_files()?;println!("Game data verified; engine: {}",s.wasm_path.display());}
        Some("--gpu-check")=>{
            let mut d=wgpu::InstanceDescriptor::new_without_display_handle();d.backends=crate::platform::gpu_backends();
            let i=wgpu::Instance::new(d);
            let adapter=pollster::block_on(i.request_adapter(&wgpu::RequestAdapterOptions::default())).context("No supported GPU adapter")?;
            println!("GPU: {:?}",adapter.get_info());
            let (_device,_queue)=pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor{label:Some("Mugoat Windows GPU check"),..Default::default()})).context("GPU device creation failed")?;
            println!("Device creation passed. Game shaders and presentation are not tested by this command.");
        }
        _=>return Ok(false)
    }Ok(true)
}
