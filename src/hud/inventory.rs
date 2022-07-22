use crate::{prelude::*, GameAssets};

#[derive(Component)]
pub struct InventoryNode;

#[derive(Component)]
pub struct InventoryText;

pub fn create_inventory(mut commands: Commands, _tile_params: Res<TileParams>, assets: Res<GameAssets>) {
    info!("creating inventory");
    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                // size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                size: Size::new(Val::Auto, Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(HEALTH_BAR_SIZE),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(InventoryNode)
        .with_children(|commands| {
            commands
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Inventory",
                        TextStyle {
                            color: Color::YELLOW.into(),
                            font: assets.font.clone(),
                            font_size: HEALTH_BAR_SIZE,
                            ..default()
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Left,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(InventoryText);
        });
}

pub fn add_items(
    mut commands: Commands,
    items: Query<(Entity, &Name), Added<Carried>>,
    inv: Query<Entity, With<InventoryNode>>,
    assets: Res<GameAssets>,
) {
    let inv = inv.single();
    for (item, name) in items.iter() {
        info!("adding {item:?} to inventory");

        commands
            .entity(item)
            .insert_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: String::default(),
                            style: TextStyle {
                                color: Color::WHITE.into(),
                                font: assets.font.clone(),
                                font_size: HEALTH_BAR_SIZE,
                                ..default()
                            },
                        },
                        TextSection {
                            value: format!(" : {name}"),
                            style: TextStyle {
                                color: Color::WHITE.into(),
                                font: assets.font.clone(),
                                font_size: HEALTH_BAR_SIZE,
                                ..default()
                            },
                        },
                    ],
                    alignment: TextAlignment {
                        horizontal: HorizontalAlign::Left,
                        vertical: VerticalAlign::Center,
                    },
                },
                ..Default::default()
            })
            .insert(InventoryText);

        commands.entity(inv).add_child(item);
    }
}

pub fn item_numbers(
    inventory: Query<&Children, (With<InventoryNode>, Changed<Children>)>,
    mut nodes: Query<&mut Text, With<InventoryText>>,
) {
    inventory.for_each(|children| {
        let mut i = 1;
        for childid in children.iter().skip(1) {
            if let Ok(mut child) = nodes.get_mut(*childid) {
                child.sections[0].value = i.to_string();
                i += 1;
            }
        }
    })
}
