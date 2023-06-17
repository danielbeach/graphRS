from pyvis.network import Network
import csv

def read_csv_to_list(filename):
    nodes = []
    with open(filename, newline='') as f:
        reader = csv.reader(f)
        headers = next(reader, None)
        for row in reader:
            for r in row:
                nodes.append(r)
    return nodes


def read_csv_nodes(filename):
    with open(filename, newline='') as f:
        reader = csv.reader(f)
        headers = next(reader, None)
        for row in reader:
                yield row


def main():
    nodes = read_csv_to_list('nodes.csv')
    net = Network()
    for node in nodes:
        net.add_node(node)
    edges = read_csv_nodes('edges.csv')
    for edge in edges:
        if len(edge[0]) > 1 and len(edge[1]) > 1:
            net.add_edge(edge[0], edge[1])

    net.show('mygraph.html')

if __name__ == "__main__":
    main()