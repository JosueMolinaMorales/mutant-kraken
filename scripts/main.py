import json
import os
import csv


def mutation_count_for_files(file_mutations):
    mutations_per_file = {}
    for (file_name, objs) in file_mutations.items():
        if file_name not in mutations_per_file:
            mutations_per_file[file_name] = []
        for val in objs.values():
            mutations_per_file[file_name].extend(val["mutations"])

    # Save to csv
    with open('mutation_count_per_file.csv', 'w', newline='', encoding="UTF-8") as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(['File Name', 'Mutation Count'])
        for (file_name, mutations) in mutations_per_file.items():
            writer.writerow([file_name, len(mutations)])


def main():
    # For every file in the outputs folder, parse the file as a json object
    # and print the contents to the console
    objs = []
    file_mutations = {}
    for filename in os.listdir('outputs'):
        with open(os.path.join('outputs', filename), 'r') as f:
            data = json.load(f)
            objs.append(data)
            file_mutations[filename] = data

    mutation_count_for_files(file_mutations)
    # mutations = []
    # for obj in objs:
    #     for (key, value) in obj.items():
    #         mutations.extend(value["mutations"])

    # operators = {}
    # for mutation in mutations:
    #     operator = mutation["mutation_type"]
    #     if operator in operators:
    #         operators[operator] += 1
    #     else:
    #         operators[operator] = 1
    # print(len(mutations))
    # for (key, value) in operators.items():
    #     print(key, value)

    # # Create a bar chart of the data
    # # where the x-axis is the mutation operator verticle (to fit) and the y-axis is the number of times the operator was used
    # import matplotlib.pyplot as plt

    # # Extract keys and values from the dictionary
    # mutation_names = list(operators.keys())
    # mutation_counts_values = list(operators.values())

    # # Plotting the bar graph with rotated x-axis labels
    # plt.bar(mutation_names, mutation_counts_values, color='blue')
    # # Rotate x-axis labels for better visibility
    # plt.xticks(rotation=45, ha='right')

    # # Adding labels and title
    # plt.xlabel('Mutation Operator')
    # plt.ylabel('Count')
    # plt.title('Mutation Operator Counts')

    # # Adjust layout to prevent clipping of labels
    # plt.tight_layout()    # Plotting the bar graph
    # plt.bar(mutation_names, mutation_counts_values, color='blue')

    # # Adding labels and title
    # plt.ylabel('Count')
    # plt.title('Mutation Operator Counts')

    # # Save the figure
    # plt.savefig('mutation_operator_counts.png')

    # # Create a csv file with the data
    # with open('mutation_operator_counts.csv', 'w', newline='', encoding="UTF-8") as csvfile:
    #     writer = csv.writer(csvfile)
    #     writer.writerow(['Mutation Operator', 'Count'])
    #     for (key, value) in operators.items():
    #         writer.writerow([key, value])


if __name__ == '__main__':
    main()
