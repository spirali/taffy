#[test]
fn grid_auto_single_item_fixed_width_with_definite_width() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node6 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node7 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node8 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![points(40f32), points(40f32), points(40f32)],
                grid_template_columns: vec![points(40f32), auto(), auto()],
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(200f32), height: auto() },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5, node6, node7, node8],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 200f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 120f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 170f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node3).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node3).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node3).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node3).unwrap().location.y, 40f32);
    assert_eq!(taffy.layout(node4).unwrap().size.width, 130f32);
    assert_eq!(taffy.layout(node4).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node4).unwrap().location.x, 40f32);
    assert_eq!(taffy.layout(node4).unwrap().location.y, 40f32);
    assert_eq!(taffy.layout(node5).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node5).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node5).unwrap().location.x, 170f32);
    assert_eq!(taffy.layout(node5).unwrap().location.y, 40f32);
    assert_eq!(taffy.layout(node6).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node6).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node6).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node6).unwrap().location.y, 80f32);
    assert_eq!(taffy.layout(node7).unwrap().size.width, 130f32);
    assert_eq!(taffy.layout(node7).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node7).unwrap().location.x, 40f32);
    assert_eq!(taffy.layout(node7).unwrap().location.y, 80f32);
    assert_eq!(taffy.layout(node8).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node8).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node8).unwrap().location.x, 170f32);
    assert_eq!(taffy.layout(node8).unwrap().location.y, 80f32);
}
