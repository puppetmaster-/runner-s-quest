
pub mod pyxeledit;
pub mod helper;

use std::collections::HashMap;
use crate::*;
use crate::tilemap::pyxeledit::PyxelTilemap;
use crate::utils::vecgrid::VecGrid;

#[allow(dead_code)]
impl Tilemap{
    /// create empty tilemap
    /// use add_tiles_from_map or set_tiles to fill
    pub fn new(tileset_rect: Rect, tile_width: i32, tile_height: i32, width: usize, height: usize) -> Tilemap{
        Tilemap{
            width,
            height,
            viewport: DEFAULT_RECTANGLE,
            tile_height,
            tile_width,
            layers: vec![Layer {tiles: VecGrid::new(width, height),..Layer::default()}],
            tile_rectangles: get_tile_rectangles(tileset_rect, tile_width, tile_height),
            layer_to_draw: DEFAULT_LAYER_TO_DRAW,
        }
    }

    //OK
    pub fn from_pyxeledit(clip: Rect, data: &str) -> Tilemap{
        let pyxeltilemap = PyxelTilemap::new(data);
        transform_pyxeltilemap(clip, pyxeltilemap)
    }

    //OK
    pub fn color(&mut self, layer: usize, color: Color) ->&Tilemap{
        match self.layers.get_mut(layer){
            None => self,
            Some(l) => {
                l.color = color;
                self
            },
        }
    }

    /*
    /// just a map with tile ids
    /// neither rotation nor flipping
    pub fn set_tiles_from_map(&mut self, layer: usize, list: &[Vec<u32>]){
        let tiles = self.create_tiles_from_map(list);
        match  self.layers.get_mut(layer){
            None => self.add_layer(tiles),
            Some(layer) => layer.tiles = tiles,
        }
    }
     */

    /*
    pub fn viewport(&mut self, rectangle: Rect) -> &Tilemap{
        self.viewport = rectangle;
        self
    }
     */

    //TODO
    pub fn get_all_position_from_id(&self,layer: usize, id: u32) -> Vec<Vec2>{
        let mut positions = Vec::new();
        if let Some(layer) = self.layers.get(layer) {
            let tiles = &*layer.tiles.get_data();
            for (_i,t) in tiles.iter().enumerate(){
                if t.is_some() && t.as_ref().unwrap().id == id{
                    let pos = Vec2::new(t.as_ref().unwrap().position_x,t.as_ref().unwrap().position_y * -1.0);
                    positions.push(pos);
                    //let x = (i % self.width) * self.tile_width as usize;
                    //let y = ((i / self.width)+1) * self.tile_height as usize;
                    //positions.push(Vec2::new((x+1) as f32,(y-1) as f32));
                }
            }
        };
        positions
    }

    //TODO
    pub fn replace_all_tileid(&mut self, layer: usize, old_id: u32, new_id: Option<u32>){
        if let Some(layer) = self.layers.get_mut(layer) {
            for x in 0..self.height {
                for y in 0..self.width {
                    if let Some(tile) = layer.tiles.get_mut(x, y) {
                        if tile.id == old_id {
                            if let Some(id) = new_id {
                                tile.id = id;
                            } else {
                                layer.tiles.delete(x, y);
                            }
                        }
                    }
                }
            }
        }else{

        }
    }

    //TODO
    pub fn set_new_id_at(&mut self, layer: usize, new_id: u32, position: Vec2){
        let x = position.x  / self.tile_width as f32;
        let y = (position.y*-1.0)  / self.tile_height as f32;
        if let Some(layer) = self.layers.get_mut(layer){
            match layer.tiles.get_mut(x as usize, y as usize){
                None => layer.tiles.set(Tile{
                    id: new_id,
                    x: x as i32,
                    y: y as i32,
                    position_x: (x as i32 * self.tile_width) as f32,
                    position_y: (y as i32 * self.tile_height) as f32,
                    ..Tile::default()
                }, x as usize, y as usize),
                Some(tile) => tile.id = new_id,
            };
        }else{

        }
    }

    //OK
    pub fn visibility(&mut self, layer: usize, visibility: bool){
        if let Some(l) = self.layers.get_mut(layer) {
            l.visibility = visibility
        }else{

        }
    }

    //OK
    pub fn get_layer_id(&self, name: &str) ->usize{
        for (i, layer) in self.layers.iter().enumerate(){
            if layer.name.eq(name){
                return i;
            }
        }
        99
    }

    //OK
    pub fn get_layer_name(&self, layer: usize) ->&str{
        if let Some(layer) = self.layers.get(layer as usize){
            &layer.name
        }else{
            ""
        }
    }

    //OK
    pub fn get_id_at_position(&self, layer: usize, position: Vec2) -> Option<u32>{
        let x = position.x  / self.tile_width as f32;
        let y = (position.y*-1.0)  / self.tile_height as f32;
        self.get_id_at(layer, x as usize, y as usize)
    }

    //OK
    pub fn get_rect_at_position(&self, position: Vec2) -> Rect{
        let x = position.x  / self.tile_width as f32;
        let y = (position.y*-1.0)  / self.tile_height as f32;
        let x_pos = x as i32 * self.tile_width;
        let y_pos = y as i32 * self.tile_height;
        Rect::new(x_pos as f32, y_pos as f32, self.tile_width as f32, self.tile_height as f32)
    }

    //OK
    pub fn get_id_at(&self, layer_nr: usize, x: usize,y: usize) -> Option<u32>{
        match self.layers.get(layer_nr) {
            None => {
                None
            },
            Some(layer) => {
                match layer.tiles.get(x,y){
                    None => {
                        None
                    },
                    Some(tile) => Some(tile.id)
                }
            },
        }
    }

    //OK
    pub fn get_clip_from_id(&self, id: u32) -> Rect{
        self.tile_rectangles[&id]
    }

    //OK
    pub fn get_irect_from_id(&self, id: u32) -> IRect{
        let rect = self.tile_rectangles[&id];
        IRect{
            offset: IVec2::new(rect.x as i32, rect.y as i32),
            size: IVec2::new(16,16),
        }
    }

    //OK
    pub fn draw_layer(&mut self, texture: TextureHandle, pos: Vec2, layer_to_draw: usize, z_index: i32, tint: Color) {
        self.draw(texture, pos,Some(layer_to_draw),z_index, tint);
    }

    //OK
    pub fn draw(&self, texture: TextureHandle, pos: Vec2, layer_to_draw: Option<usize>, z_index: i32, tint: Color) {
        for (i, layer) in self.layers.iter().enumerate() {
            if layer.visibility && layer_to_draw.is_none() || layer_to_draw.is_some() && i == layer_to_draw.unwrap(){
                for tile in layer.tiles.get_data().iter().filter(|t| t.is_some()) {
                    match tile {
                        None => (),
                        Some(tile) => {
                            let tmp_pos = Vec2::new(pos.x + tile.position_x, (pos.y + tile.position_y) * -1.0);
                                draw_sprite_ex(texture, tmp_pos, tint, z_index,
                                               DrawTextureParams{
                                                   dest_size: Some(vec2(16.0, 16.0).as_world_size()),
                                                   source_rect: Some(self.get_irect_from_id(tile.id)),
                                                ..Default::default()
                               });

                        }
                    }
                }
            }
        }
    }
    //TODO
    fn is_inside_viewport(&self, position: Vec2) -> bool{
        !(position.x < self.viewport.x ||
            position.y < self.viewport.y ||
            position.x > self.viewport.x + self.viewport.w ||
            position.y > self.viewport.y + self.viewport.h
        )
    }

    //TODO
    pub fn get_frames_from_ids(&self, ids: &[u32]) -> Vec<Rect>{
        let mut frames = Vec::with_capacity(ids.len());
        for id in ids{
            frames.push(self.tile_rectangles[&(id)]);
        }
        frames
    }

    //TODO
    fn create_tiles_from_map(&mut self, list: &[Vec<u32>])->VecGrid<Tile>{
        let mut tiles = VecGrid::new(list.len(), list[0].len());
        for (x,row) in list.iter().enumerate() {
            for (y,id) in row.iter().enumerate(){
                tiles.set(Tile {
                    id: *id,
                    x: x as i32,
                    y: y as i32,
                    position_x: (x as i32 * self.tile_width) as f32,
                    position_y: (y as i32 * self.tile_height) as f32,
                    ..Tile::default()
                },x,y);
            };
        }
        tiles
    }

    //TODO
    fn add_layer(&mut self, tiles: VecGrid<Tile>){
        let layer = Layer{
            tiles,
            ..Layer::default()
        };
        self.layers.push(layer);
    }
}




#[allow(dead_code)]
pub struct Tilemap {
    width: usize,
    height: usize,
    viewport: Rect,
    tile_height: i32,
    tile_width: i32,
    layers: Vec<Layer>,
    tile_rectangles: HashMap<u32, Rect>,
    layer_to_draw: i64,
}

pub struct Layer {
    tiles: VecGrid<Tile>,
    name: String,
    visibility: bool,
    color: Color,
}

#[allow(dead_code)]
pub struct Tile {
    id: u32,
    x: i32,
    y: i32,
    position_x: f32,
    position_y: f32,
    rotation: f32,
    scale: Vec2,
}

fn get_tile_rectangles(clip: Rect, tile_width: i32, tile_height: i32) ->HashMap<u32, Rect>{
    let mut id = 0;
    let x = clip.h as i32 / tile_width;
    let y = clip.w as i32 / tile_height;
    let mut tile_rectangles: HashMap<u32, Rect> = HashMap::with_capacity((x * y) as usize);
    for i in 0..x{
        for j in 0..y{
            let rec = Rect::new(clip.x +(j*tile_width) as f32,clip.y +(i*tile_height) as f32, tile_width as f32, tile_height as f32); //switch x and y axis
            tile_rectangles.insert(id,rec);
            id +=1;
        }
    }
    tile_rectangles
}

fn transform_pyxeltilemap(clip: Rect, pyxeltilemap: PyxelTilemap) ->Tilemap{
    Tilemap{
        width: pyxeltilemap.tileswide as usize,
        height: pyxeltilemap.tileshigh as usize,
        viewport: DEFAULT_RECTANGLE,
        tile_height: pyxeltilemap.tile_height,
        tile_width: pyxeltilemap.tile_width,
        layers: transform_pyxellayer(&pyxeltilemap.layers, pyxeltilemap.tileswide as usize, pyxeltilemap.tileshigh as usize),
        tile_rectangles: get_tile_rectangles(clip, pyxeltilemap.tile_width, pyxeltilemap.tile_height),
        layer_to_draw: DEFAULT_LAYER_TO_DRAW,
    }
}

fn transform_pyxellayer(pyxellayers: &[pyxeledit::Layers], width: usize, height: usize) ->Vec<Layer>{
    let mut layers: Vec<Layer> = Vec::new();
    for pyxellayer in pyxellayers.iter().rev(){
        let l = Layer{
            tiles: transform_pyxeltile(&pyxellayer.tiles, width, height),
            name: pyxellayer.name.clone(),
            ..Layer::default()
        };
        layers.push(l);
    }
    layers
}

fn transform_pyxeltile(pyxeltiles: &[pyxeledit::Tile], width: usize, height: usize) -> VecGrid<Tile>{
    let mut vecgrid: VecGrid<Tile> = VecGrid::new(width, height);
    for t in pyxeltiles.iter(){
        let tile = Tile{
            id: t.id as u32,
            x: t.x,
            y: t.y,
            position_x: t.position_x,
            position_y: t.position_y,
            rotation: t.rotation,
            scale: Vec2::new(t.scale.0,t.scale.1),
        };
        vecgrid.set(tile, t.x as usize, t.y as usize);
    };
    vecgrid
}


impl Default for Layer {
    fn default() -> Layer {
        Layer{
            tiles: VecGrid::new(1,1),
            name: "".to_string(),
            visibility: true,
            color: WHITE,
        }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile{
            id: 0,
            x: 0,
            y: 0,
            position_x: 0.0,
            position_y: 0.0,
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        }
    }
}

const DEFAULT_RECTANGLE: Rect = Rect{ x: 0.0, y: 0.0, w: 0.0, h: 0.0 };
const DEFAULT_LAYER_TO_DRAW: i64 = -1;