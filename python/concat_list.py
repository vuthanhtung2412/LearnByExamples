# Create two lists
list1 = [1, 2, 3]
list2 = [4, 5, 6]

# Create another reference to list1
list_ref = list1

# Extend list1 using += operator
list1 += list2
print(f"List1 after +=: {list1}")  # [1, 2, 3, 4, 5, 6]
print(f"List reference after +=: {list_ref}")  # [1, 2, 3, 4, 5, 6]

# Reset lists
list1 = [1, 2, 3]
list_ref = list1

# Extend list1 using +
list1 = list1 + list2
print(f"List1 after +: {list1}")  # [1, 2, 3, 4, 5, 6]
print(f"List reference after +: {list_ref}")  # [1, 2, 3]
