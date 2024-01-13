import json
import os


def main():
    # For every file in the outputs folder, parse the file as a json object
    # and print the contents to the console
    objs = []
    for filename in os.listdir('outputs'):
        with open(os.path.join('outputs', filename), 'r') as f:
            data = json.load(f)
            objs.append(data)

    mutations = []
    for obj in objs:
        for (key, value) in obj.items():
            mutations.extend(value["mutations"])

    operators = {}
    for mutation in mutations:
        operator = mutation["mutation_type"]
        if operator in operators:
            operators[operator] += 1
        else:
            operators[operator] = 1
    print(len(mutations))
    for (key, value) in operators.items():
        print(key, value)

    # Create a bar chart of the data
    # where the x-axis is the mutation operator verticle (to fit) and the y-axis is the number of times the operator was used
    import matplotlib.pyplot as plt

    # Extract keys and values from the dictionary
    mutation_names = list(operators.keys())
    mutation_counts_values = list(operators.values())

    # Plotting the bar graph with rotated x-axis labels
    plt.bar(mutation_names, mutation_counts_values, color='blue')
    # Rotate x-axis labels for better visibility
    plt.xticks(rotation=45, ha='right')

    # Adding labels and title
    plt.xlabel('Mutation Operator')
    plt.ylabel('Count')
    plt.title('Mutation Operator Counts')

    # Adjust layout to prevent clipping of labels
    plt.tight_layout()    # Plotting the bar graph
    plt.bar(mutation_names, mutation_counts_values, color='blue')

    # Adding labels and title
    plt.ylabel('Count')
    plt.title('Mutation Operator Counts')

    # Save the figure
    plt.savefig('mutation_operator_counts.png')


if __name__ == '__main__':
    main()
