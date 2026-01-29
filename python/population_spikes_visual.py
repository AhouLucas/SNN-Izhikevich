import matplotlib.pyplot as plt
import numpy as np

def plot_population_spikes(path_to_csv):

    data = np.loadtxt(path_to_csv, delimiter=',', skiprows=1)
    
    spike_times = data[:, 0]
    neuron_ids = data[:, 1]

    plt.figure(figsize=(10, 6))
    plt.scatter(spike_times, neuron_ids, s=2, color='black')
    plt.title('Population Spikes Raster Plot')
    plt.xlabel('Time (ms)')
    plt.ylabel('Neuron ID')
    plt.grid()
    plt.show()


if __name__ == "__main__":
    path_to_csv = 'data/chain_test.csv'
    plot_population_spikes(path_to_csv)