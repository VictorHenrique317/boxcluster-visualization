def read_file(file_name):
    with open(file_name, 'r') as file:
        lines = file.readlines()
    data = {}
    for line in lines:
        index, value = line.split(':')
        index = tuple(map(int, index.strip('[] \n').split(',')))
        value = float(value.strip(', \n'))
        data[index] = value
    return data

def compare_files(file1, file2):
    data1 = read_file(file1)
    data2 = read_file(file2)
    different_indices = []
    for index in data1:
        if index in data2 and data1[index] != data2[index]:
            different_indices.append(index)
    return different_indices

file1 = 'right.txt'
file2 = 'wrong.txt'
different_indices = compare_files(file1, file2)
print('Indices with different values:', different_indices)
