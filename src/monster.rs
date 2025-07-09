use macroquad::prelude::*;
// 引入随机数生成器相关模块
use ::rand::{Rng, SeedableRng};
use ::rand::rngs::SmallRng;
use crate::{TILE_SIZE, Tile, MAP_WIDTH, MAP_HEIGHT};

//单个怪物的结构体
#[derive(Clone)]
pub struct Monster {
    //怪物在地图的位置
    pub x: f32,
    pub y: f32,
    //怪物的样子
    pub texture: Texture2D,
}

//怪物集合的结构体
//结构体作用：管理游戏中的所有怪物，包含怪物列表、样子、随机数生成器、移动计时器和移动间隔等信息。
pub struct Monsters {
    pub list: Vec<Monster>,
    pub texture: Texture2D,
    //随机数生成器
    rng: SmallRng,
    move_timer: f32,
    move_interval: f32,
}

impl Monsters {
    //创建怪物集合实例的函数
     pub async fn new(count: usize) -> Self {
        let texture = load_texture("assets/monster.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        let mut list = Vec::new();
        let mut rng = SmallRng::from_entropy();

        for _ in 0..count {
            loop {
                //随机生成怪物的坐标
                let x = rng.gen_range(1..MAP_WIDTH - 1) as f32;
                let y = rng.gen_range(1..MAP_HEIGHT - 1) as f32;

                //检查新生成的怪物位置是否与已有的怪物重叠
                if !list.iter().any(|m: &Monster| (m.x - x).abs() < 1.0 && (m.y - y).abs() < 1.0) {
                    list.push(Monster {
                        x,
                        y,
                        texture: texture.clone(),
                    });
                    break;
                }
            }
        }

        //返回Monsters实例
        Self {
            list,
            texture,
            rng,
            move_timer: 0.0,
            move_interval: 0.5,
        }
    }

    pub fn update(&mut self, delta: f32, map: &[[Tile; MAP_WIDTH]; MAP_HEIGHT]) {
        self.move_timer += delta;
        //怪物每次移动必须要间隔一个移动间隔，如果时间短于移动间隔，则不移动
        if self.move_timer < self.move_interval {
            return;
        }
        //重置移动计时器
        self.move_timer = 0.0;

        //怪物可以移动的方向（“米”字形）
        let directions = [
            (0.0, -1.0),
            (0.0, 1.0),
            (-1.0, 0.0),
            (1.0, 0.0),
            (-1.0, -1.0),
            (1.0, 1.0),
            (-1.0, 1.0),
            (1.0, -1.0),
        ];

        //获取当前所有怪物的位置
        let positions: Vec<(f32, f32)> = self.list.iter().map(|m| (m.x, m.y)).collect();
        let mut new_positions: Vec<(f32, f32)> = Vec::with_capacity(self.list.len());

        for i in 0..self.list.len() {
            let mut attempts = 0;
            loop {
                //随机选择一个移动方向
                let dir = directions[self.rng.gen_range(0..directions.len())];
                let new_x = positions[i].0 + dir.0;
                let new_y = positions[i].1 + dir.1;

                //检查新位置是否越界
                if new_x < 1.0 || new_x >= (MAP_WIDTH - 1) as f32 || new_y < 1.0 || new_y >= (MAP_HEIGHT - 1) as f32 {
                    attempts += 1;
                    if attempts > 10 { break; }
                    continue;
                }

                //检查新位置是否是墙壁
                if map[new_y as usize][new_x as usize] == Tile::Wall {
                    attempts += 1;
                    if attempts > 10 { break; }
                    continue;
                }

                //检查新位置是否与其他怪物重叠
                let overlap = positions.iter().enumerate().any(|(j, &(x, y))| {
                    if i == j { return false; }
                    (x - new_x).abs() < 1.0 && (y - new_y).abs() < 1.0
                }) || new_positions.iter().any(|&(x, y)| {
                    (x - new_x).abs() < 1.0 && (y - new_y).abs() < 1.0
                });

                if !overlap {
                    //更新怪物位置
                    self.list[i].x = new_x;
                    self.list[i].y = new_y;
                    new_positions.push((new_x, new_y));
                    break;
                } else {
                    attempts += 1;
                    if attempts > 10 { break; }
                }
            }
        }
    }

    //负责将所有怪物绘制到游戏主页面的函数
    pub fn draw(&self) {
        for monster in &self.list {
            draw_texture(
                &monster.texture,
                monster.x * TILE_SIZE,
                monster.y * TILE_SIZE,
                WHITE,
            );
        }
    }
}
