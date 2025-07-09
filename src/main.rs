mod hud;
mod monster;
mod health_pack;
mod menu;
mod game_over;

//引入macroquad 库的预导入模块和数学模块
use macroquad::prelude::*;
use macroquad::math::Vec2; 
//引入main函数中需要使用的所有其他本项目自定义的模块中含有的结构体or函数
use hud::{draw_health_bar, draw_message};
use monster::Monsters;
use health_pack::HealthPacks;
use menu::{Menu, GameDifficulty};
use game_over::GameOver;

//地图格子的大小
const TILE_SIZE: f32 = 64.0;
//游戏地图大小
const MAP_WIDTH: usize = 12;
const MAP_HEIGHT: usize = 10;

//地图的格子有两种类型：一种是墙壁、一种是地板
#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

//玩家结构体
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub health: i32,
    pub texture: Texture2D,
    pub health_packs_collected: usize,
    pub monster_attacks: usize,
}

//玩家结构体的方法
impl Player {
    //玩家受到怪物攻击的方法
    pub fn take_damage(&mut self, amount: i32) {
        println!("玩家受到伤害: {} HP", amount);
        self.health -= amount;
        self.monster_attacks += 1;
        if self.health < 0 {
            self.health = 0;
        }
    }

    //玩家吃血包回血的方法
    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
        self.health_packs_collected += 1;
        if self.health > 100 {
            self.health = 100;
        }
    }
}

// 绘制按钮的函数
fn draw_button(pos: Vec2, size: Vec2, label: &str, font_size: Option<u16>) -> bool {
    let button_color = Color::new(0.2, 0.2, 0.2, 1.0);
    let hover_color = Color::new(0.3, 0.3, 0.3, 1.0);
    let text_color = WHITE;

    let mouse_pos = mouse_position();
    let button_rect = Rect::new(pos.x, pos.y, size.x, size.y);
    let is_hovered = button_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));

    draw_rectangle(pos.x, pos.y, size.x, size.y, if is_hovered { hover_color } else { button_color });
    
    let text_size = measure_text(label, None, font_size.unwrap_or(20), 1.0);
    draw_text_ex(
        label,
        pos.x + size.x / 2.0 - text_size.width / 2.0,
        pos.y + size.y / 2.0 + text_size.height / 2.0,
        TextParams {
            font_size: font_size.unwrap_or(20),
            color: text_color,
            ..Default::default()
        },
    );

    is_hovered && is_mouse_button_pressed(MouseButton::Left)
}

//定义游戏结构体
pub struct Game {
    map: [[Tile; MAP_WIDTH]; MAP_HEIGHT], // 地图二维数组
    wall_texture: Texture2D, // 墙壁的纹理
    floor_texture: Texture2D, // 地板的纹理
    player: Player, // 玩家对象
    monsters: Monsters, // 怪物集合
    health_packs: HealthPacks, // 血包集合
    pub message: Option<(String, Color)>, // 游戏中的提示消息
    message_timer: f32, // 消息显示的计时器
    damage_cooldown: f32, // 玩家受到伤害后的冷却时间
    game_time: f32, // 游戏进行的时间
    pub paused: bool, // 游戏是否暂停

}

//游戏结构体的相关方法
impl Game {
    //异步创建游戏的实例的方法
    pub async fn new(difficulty: GameDifficulty) -> Self {
        //墙壁的图片加载
        let wall_texture = load_texture("assets/wall.png").await.unwrap();
        wall_texture.set_filter(FilterMode::Nearest);

        //地板的图片加载
        let floor_texture = load_texture("assets/floor.png").await.unwrap();
        floor_texture.set_filter(FilterMode::Nearest);

        //玩家的图片加载
        let player_texture = load_texture("assets/player.png").await.unwrap();
        player_texture.set_filter(FilterMode::Nearest);

        //一开始整张地图都是地板
        let mut map = [[Tile::Floor; MAP_WIDTH]; MAP_HEIGHT];
        //然后再将游戏界面的边缘变成墙壁
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_HEIGHT - 1 {
                    map[y][x] = Tile::Wall;
                }
            }
        }

        //初始化玩家对象
        let player = Player {
            x: 3,
            y: 3,
            health: 100,
            texture: player_texture,
            health_packs_collected: 0,
            monster_attacks: 0,
        };

        //根据游戏难度设置怪物的数量
        let monster_count = match difficulty {
            GameDifficulty::Easy => 1,
            GameDifficulty::Medium => 3,
            GameDifficulty::Hard => 5,
        };

        //怪物集合
        let monsters = Monsters::new(monster_count).await;
        //血包集合
        let health_packs = HealthPacks::new().await;

        //返回游戏的实例
        Self {
            map,
            wall_texture,
            floor_texture,
            player,
            monsters,
            health_packs,
            message: None,
            message_timer: 0.0,
            damage_cooldown: 0.0,
            game_time: 0.0,
            paused: false,
        }
    }


//游戏主页面暂停按钮的渲染
pub fn draw_pause_button(&mut self) {
    let label = if self.paused { "Restart" } else { "Pause" };

    let button_width = 160.0;
    let button_height = 60.0;


    let button_pos = Vec2::new(
        TILE_SIZE * (MAP_WIDTH as f32 - 2.55),  
        TILE_SIZE * 0.04 ,
    );

    if draw_button(
        button_pos,
        Vec2::new(button_width, button_height),
        label,
        Some(28),
    ) {
        self.paused = !self.paused;
    }
}
    //绘制游戏画面的方法
    pub fn draw(&mut self) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let pos_x = x as f32 * TILE_SIZE;
                let pos_y = y as f32 * TILE_SIZE;

                let texture = match self.map[y][x] {
                    Tile::Wall => &self.wall_texture,
                    Tile::Floor => &self.floor_texture,
                };

                draw_texture(texture, pos_x, pos_y, WHITE);
            }
        }
        //绘制玩家
        draw_texture(
            &self.player.texture,
            self.player.x as f32 * TILE_SIZE,
            self.player.y as f32 * TILE_SIZE,
            WHITE,
        );

        self.monsters.draw();//绘制怪物
        self.health_packs.draw();//绘制血包
        self.draw_pause_button(); //绘制暂停按钮
        //如果有游戏提示信息，绘制信息
        if let Some((msg, color)) = &self.message {
            draw_message(msg, *color);
        }
    }

    //更新游戏状态的函数（游戏的本质就是一个状态机）
    pub fn update(&mut self, delta: f32) -> Option<(f32, usize, usize)> {
        //游戏如果暂停，则状态不更新
        if self.paused {
            return None; 
        }

        //在本游戏里，玩家生存的时间是衡量玩家游戏情况的硬指标，因此需要计时
        self.game_time += delta;

        // 如果玩家生命值为 0，返回游戏时间、收集的血包数量和被攻击次数
        if self.player.health <= 0 {
            return Some((
                self.game_time,
                self.player.health_packs_collected,
                self.player.monster_attacks,
            ));
        }

        if self.damage_cooldown > 0.0 {
            self.damage_cooldown -= delta;
        }

        //玩家上下左右四个方向的移动（必须移动到地板，不可以移动到墙壁）
        if is_key_pressed(KeyCode::Left) && self.player.x > 0 {
            if self.map[self.player.y][self.player.x - 1] == Tile::Floor {
                self.player.x -= 1;
            }
        }
        if is_key_pressed(KeyCode::Right) && self.player.x < MAP_WIDTH - 1 {
            if self.map[self.player.y][self.player.x + 1] == Tile::Floor {
                self.player.x += 1;
            }
        }
        if is_key_pressed(KeyCode::Up) && self.player.y > 0 {
            if self.map[self.player.y - 1][self.player.x] == Tile::Floor {
                self.player.y -= 1;
            }
        }
        if is_key_pressed(KeyCode::Down) && self.player.y < MAP_HEIGHT - 1 {
            if self.map[self.player.y + 1][self.player.x] == Tile::Floor {
                self.player.y += 1;
            }
        }

        self.monsters.update(delta, &self.map);
        self.health_packs.update(delta, &self.map);

        if self.damage_cooldown <= 0.0 {
            for monster in &self.monsters.list {
                let mx = monster.x.floor() as usize;
                let my = monster.y.floor() as usize;

                // 如果玩家和怪物重叠，玩家受到伤害并设置冷却时间和提示消息
                if mx == self.player.x && my == self.player.y {
                    self.player.take_damage(20);
                    self.damage_cooldown = 1.0;
                    self.message = Some(("Attacked! -20HP".to_string(), RED));
                    self.message_timer = 1.0;
                    break;
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            if let Some(idx) = self.health_packs.list.iter().position(|hp| {
                hp.x == self.player.x && hp.y == self.player.y
            }) {
                //玩家生命值满了，输出提示信息提示玩家
                if self.player.health == 100 {
                    self.message = Some(("Energy is already full!".to_string(), GREEN));
                    self.message_timer = 2.0;
                } else {
                    //玩家生命值未满并且玩家与血包重叠，玩家回血，并将血包移除
                    self.player.heal(20);
                    self.health_packs.list.remove(idx);
                }
            }
        }

        if self.message.is_some() {
            self.message_timer -= delta;
            if self.message_timer <= 0.0 {
                self.message = None;
                self.message_timer = 0.0;//如果消息显示时间到，则该信息需要移除
            }
        }

        None//游戏没结束，则返回None
    }
}

#[macroquad::main("Monster Is Coming!")]
async fn main() {
    request_new_screen_size(MAP_WIDTH as f32 * TILE_SIZE, MAP_HEIGHT as f32 * TILE_SIZE);
    next_frame().await;

    //枚举定义游戏状态
    enum GameState {
        Menu(Menu),
        Playing(Game),
        GameOver(GameOver),
    }
    //游戏的初始化状态是菜单
    let mut game_state = GameState::Menu(Menu::new().await);
    //游戏初始化难度为简单
    let mut _difficulty = GameDifficulty::Easy; 

    loop {
        clear_background(BLACK);

        //帧间隔时间
        let delta = get_frame_time();

        //游戏的状态
        match &mut game_state {
            GameState::Menu(menu) => {
                //绘制菜单
                menu.draw();
                if let Some(selected_difficulty) = menu.update() {
                    _difficulty = selected_difficulty;
                    //选择了难度之后，进入游戏界面
                    game_state = GameState::Playing(Game::new(_difficulty).await);
                }
            }
            GameState::Playing(game) => {
                //绘制游戏界面
                game.draw();
                //绘制玩家血条
                draw_health_bar(game.player.health);

                if let Some((time, packs, attacks)) = game.update(delta) {
                    //如果游戏结束，进入游戏结束界面
                    game_state = GameState::GameOver(GameOver::new(time, packs, attacks).await);
                }
            }
            GameState::GameOver(game_over) => {
                //绘制游戏结束界面
                game_over.draw();
                //返回菜单
                if game_over.update() {
                    game_state = GameState::Menu(Menu::new().await);
                }
            }
        }

        next_frame().await;
    }
}