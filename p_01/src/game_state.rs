/*
File: game_state.rs v0.0.1
Author: Joshua Wise
This file currently contains the all of the important state for the game which must be persisted as long as the game is running, this includes instatiating the global World struct which contains all of the components for the entire game (entities, positions, etc).

*/

use ggez::{
    event::EventHandler,
    graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect, StrokeOptions, MeshBuilder},
    Context, GameResult,
};

use crate::{components::Grid, input_handler, GameConfig, Position, SelectionRect};

pub struct GameState {
    config: GameConfig,
    pub fullscreen: bool,
    pub selection_rect: SelectionRect,
    //TODO: create world struct
    pub left_mouse_button_pressed: bool,
    pub grid: Grid,
    pub grid_mesh: Option<Mesh>,
}

impl GameState {
    pub fn new(_ctx: &mut Context, config: GameConfig) -> GameState {
        let cell_size = config.cell_size.clone();
        let g_width = config.grid_width.clone();
        let g_height = config.grid_height.clone();

        let mut state = GameState {
            config,
            fullscreen: true,
            selection_rect: SelectionRect::new(),
            left_mouse_button_pressed: false,
            //TODO: This grid may want to exist in the world struct, but maybe not. Memory concerns etc.
            grid: Grid {
                width: g_width,
                height: g_height,
                cell_size,
                // cells: Vec::from([]),
            },
            grid_mesh: None,
        };

    state.create_grid_mesh(_ctx);
        state
    }
    //TODO: put all this grid instatiation stuff into the components file or something. Possibly put it in the impl of the grid?? Maybe, maybe not.
    // TODO: make some way to keep track of cell positions and possibly IDs and stuff Prove this system by being able to select cells.
    pub fn create_grid_mesh(&mut self, ctx: &mut Context) {
        let mut mesh_builder = MeshBuilder::new();

        for row in 0..self.grid.height {
            for col in 0..self.grid.width {
                let x = col as f32 * self.grid.cell_size;
                let y = row as f32 * self.grid.cell_size;

                let rect = Rect::new(x, y, self.grid.cell_size, self.grid.cell_size);
                let _filled_cell = Mesh::new_rectangle(
                    ctx,
                    DrawMode::Fill(FillOptions::DEFAULT),
                    rect,
                    Color::from_rgba(100, 50, 40, 100),
                );

                mesh_builder.rectangle(
                    DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)), // Adjust the line width as needed
                    rect,
                    Color::from_rgba(100, 50, 40, 85), // Outline color
                ).unwrap();
            }
        }
        let mesh_data = mesh_builder.build();
        self.grid_mesh = Some(Mesh::from_data(ctx, mesh_data));
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let _delta = ctx.time.delta().as_secs_f32();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgba(9, 10, 20, 255));

      if let Some(grid_mesh)  = &self.grid_mesh {
        canvas.draw(grid_mesh, DrawParam::default());
      } 

        if self.left_mouse_button_pressed {
            let (start_x, start_y) = (self.selection_rect.start.x, self.selection_rect.start.y);
            let (end_x, end_y) = (self.selection_rect.end.x, self.selection_rect.end.y);
            let rect = Rect::new(start_x, start_y, end_x - start_x, end_y - start_y);

            let rendered_selection_rect = Mesh::new_rectangle(
                ctx,
                DrawMode::Fill(FillOptions::DEFAULT),
                rect,
                Color::from_rgba(87, 114, 119, 100), //semi-transparent green
            )?;
            canvas.draw(&rendered_selection_rect, DrawParam::default());
        }

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> ggez::GameResult<()> {
        input_handler::handle_mouse_down_event(self, button, x, y)?;
        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        input_handler::handle_mouse_up_event(self, button, _x, _y)?;
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
        if self.left_mouse_button_pressed {
            self.selection_rect.end = Position { x, y };
        }
        Ok(())
    }
}
