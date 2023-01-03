use std::fs::File;
use std::io::Write;

use linfa::prelude::*;
use linfa_trees::{DecisionTree, SplitQuality};
use ndarray::prelude::*;
use ndarray::Array2;

fn main() {
    let data: Array2<f32> = array!(
        [1., 0., 0., 800., 10000.],
        [0., 1., 1., 550., 12000.],
        [1., 1., 0., 690., 16000.],
        [1., 1., 0., 410., 11000.],
        [1., 0., 1., 630., 9000.],
        [1., 0., 0., 510., 8000.],
        [0., 1., 0., 390., 14000.],
        [1., 1., 1., 490., 19000.],
        [1., 1., 0., 230., 11000.],
        [1., 0., 0., 550., 12000.],
        [0., 1., 1., 340., 16000.],
        [0., 0., 0., 630., 18000.],
        [1., 0., 1., 790., 7000.],
        [1., 1., 1., 910., 5000.],
        [0., 0., 0., 160., 12000.],
        [1., 1., 0., 230., 12000.],
        [1., 0., 0., 340., 18000.],
        [1., 1., 0., 810., 16000.],
        [0., 1., 1., 450., 13000.],
        [1., 1., 0., 330., 17000.],
        [0., 1., 0., 270., 18000.],
        [1., 0., 0., 650., 16000.],
        [1., 1., 1., 870., 15000.],
        [1., 1., 0., 650., 14000.],
        [1., 0., 1., 540., 67000.],
        [0., 1., 0., 860., 21000.],
        [1., 1., 0., 640., 12000.],
    );

    let feature_names = vec![
        "Holiday",
        "Event",
        "Good Weather",
        "Cars at 8 AM",
        "Total Visitors",
    ];

    let num_features = data.len_of(Axis(1)) - 1;

    let features = data.slice(s![.., 0..num_features]).to_owned();

    let labels = data.column(num_features).to_owned();

    let linfa_dataset = Dataset::new(features, labels)
        .map_targets(|x| match x.to_owned() as i32 {
            i32::MIN..=10000 => "Normal",
            10001..=15000 => "Busy",
            15001..=i32::MAX => "Very Busy",
        })
        .with_feature_names(feature_names);

    let model = DecisionTree::params()
        .split_quality(SplitQuality::Gini)
        .fit(&linfa_dataset)
        .unwrap();

    File::create("DecisionTree.tex")
        .unwrap()
        .write_all(model.export_to_tikz().with_legend().to_string().as_bytes())
        .unwrap();
}
