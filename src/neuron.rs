pub struct Neuron {
    pub weights: [f64; 2],
    pub bias: f64,
}

impl Neuron {
    pub fn new() -> Self {
        Self {
            weights: [0.0, 0.0],
            bias: 0.0,
        }
    }

    pub fn forward(&self, inputs: [f64; 2]) -> f64 {
        let sum: f64 = inputs[0] * self.weights[0] + inputs[1] * self.weights[1] + self.bias;

        if sum >= 0.0 { 1.0 } else { 0.0 }
    }

    pub fn train(&mut self, inputs: [f64; 2], target: f64, learning_rate: f64) {
        let prediction: f64 = self.forward(inputs);
        let error: f64 = target - prediction;

        for i in 0..2 {
            self.weights[i] += learning_rate * error * inputs[i];
        }

        self.bias += learning_rate * error;
    }
}
