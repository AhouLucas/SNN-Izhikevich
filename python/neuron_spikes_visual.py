import matplotlib.pyplot as plt
import numpy as np


def plot_neuron_spikes(path_to_csv):

    data = np.loadtxt(path_to_csv, delimiter=',', skiprows=1)
    time = data[:, 0]
    voltage = data[:, 1]
    spikes = data[:, 2]

    plt.figure(figsize=(10, 6))
    plt.plot(time, voltage, label='Membrane Potential (mV)', color='blue')
    plt.scatter(time[spikes == 1], voltage[spikes == 1], color='red', label='Spikes', zorder=5)
    plt.title('Neuron Membrane Potential and Spikes Over Time')
    plt.xlabel('Time (ms)')
    plt.ylabel('Membrane Potential (mV)')
    plt.legend()
    plt.grid()
    plt.show()


if __name__ == "__main__":
    path_to_csv = 'data/neuron_output.csv'
    plot_neuron_spikes(path_to_csv)