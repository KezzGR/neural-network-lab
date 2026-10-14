use std::io;

use crate::neuron::Neuron;

mod neuron;

fn main() {
    let dataset = [
        ([0.0, 0.0], 0.0),
        ([1.0, 0.0], 1.0),
        ([0.0, 1.0], 1.0),
        ([1.0, 1.0], 1.0),
    ];

    let epochs = 10;
    let learning_rate = 0.1;
    let mut end_learning;

    let mut neuron: Neuron = Neuron::new();

    for i in 0..epochs {
        end_learning = true;

        println!("Epochs: {:?}", i + 1);
        for (inputs, target) in dataset {
            println!("Inputs: {:?}", inputs);

            let prediction = neuron.forward(inputs);

            println!("Target: {:?}", target);
            println!("Prediction: {:?}", prediction);
            println!("Weights: {:?}", neuron.weights);
            println!("Bias: {:?}", neuron.bias);
            println!();

            if target != prediction {
                end_learning = false;
            }

            println!("Click ENTER to continue learning.");
            io::stdin().read_line(&mut String::new()).unwrap();

            neuron.train(inputs, target, learning_rate);
            println!();
        }

        if end_learning {
            break;
        }
    }

    for (inputs, target) in dataset {
        let output = neuron.forward(inputs);
        println!("{inputs:?} => {output:?}, expected: {target:?}");
    }
}
