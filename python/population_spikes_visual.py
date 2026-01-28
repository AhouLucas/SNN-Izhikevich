import matplotlib.pyplot as plt
import numpy as np

def plot_population_spikes(path_to_csv):
    
    params = np.loadtxt(path_to_csv, delimiter=',', skiprows=1, max_rows=1)
    n, steps, dt = int(params[0]), int(params[1]), params[2]

    data = np.loadtxt(path_to_csv, delimiter=',', skiprows=3)
    
    spike_times = data[:, 0]
    neuron_ids = data[:, 1]

    plt.figure(figsize=(10, 6))
    plt.scatter(spike_times, neuron_ids, s=1, color='black')
    plt.title(f'Population Spikes Raster Plot (n={n}, steps={steps}, dt={dt} ms)')
    plt.xlabel('Time (ms)')
    plt.ylabel('Neuron ID')
    plt.xlim(0, steps * dt)
    plt.ylim(-1, n)
    plt.grid()
    plt.show()


if __name__ == "__main__":
    path_to_csv = 'data/raster.csv'
    plot_population_spikes(path_to_csv)