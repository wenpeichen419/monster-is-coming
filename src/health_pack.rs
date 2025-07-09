use macroquad::prelude::*;
use crate::{TILE_SIZE, Tile, MAP_WIDTH, MAP_HEIGHT};

//单个血包的结构体
//作用：用来表示游戏中的单个血包，包含血包的位置和样子。
pub struct HealthPack {
    pub x: usize,
    pub y: usize,
    pub texture: Texture2D,
}

//血包集合的结构体
//作用：管理游戏中的所有血包，包含血包列表、计时器和血包样子等信息。
pub struct HealthPacks {
    pub list: Vec<HealthPack>,
    pub timer: f32,
    pub texture: Texture2D,
}

impl HealthPacks {
    //创建血包集合的函数
    pub async fn new() -> Self {
        let texture = load_texture("assets/health_pack.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        //返回血包集合的实例
        Self {
            list: vec![],
            timer: 0.0,
            texture,
        }
    }

 pub fn update(&mut self, delta: f32, _map: &[[Tile; MAP_WIDTH]; MAP_HEIGHT]) {
    //更新血包生成的计时器
    self.timer += delta;
    //如果计时器达到5秒，生成血包or更新当前血包的位置
    if self.timer >= 5.0 {
        //重置血包计时器
        self.timer = 0.0;

        //随机生成血包的坐标
        let new_x = rand::gen_range(1, MAP_WIDTH - 1);
        let new_y = rand::gen_range(1, MAP_HEIGHT - 1);

        //如果血包列表空了，则创建一个新的血包
        if self.list.is_empty() {
            self.list.push(HealthPack {
                x: new_x,
                y: new_y,
                texture: self.texture.clone(),
            });
        } else {
            // 刷新已有血包位置
            self.list[0].x = new_x;
            self.list[0].y = new_y;
        }
    }
}

    //绘制血包的函数，负责将血包显示在屏幕
    pub fn draw(&self) {
        for pack in &self.list {
            draw_texture(
                &pack.texture,
                pack.x as f32 * TILE_SIZE,
                pack.y as f32 * TILE_SIZE,
                WHITE,
            );
        }
    }
}
