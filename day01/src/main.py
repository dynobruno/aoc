class Node:
    def __init__(self, char) -> None:
        self.char = char
        self.children = {}
        self.number = None

    def __str__(self) -> str:
        return f"Node(char={self.char},children={self.children.keys()},number={self.number})"

def build_trie():
    words = [
        ['one','1'],
        ['two','2'],
        ['three','3'],
        ['four','4'],
        ['five','5'],
        ['six','6'],
        ['seven','7'],
        ['eight','8'],
        ['nine','9'],
    ]
    trie = Node("")

    for (word,number) in words:
        node = trie
        for c in word:
            if c not in node.children:
                node.children[c] = Node(c)
            node = node.children[c]
        node.number = number

    return trie

input = open("/home/bruno/workspace/advent_of_code/day01/input1.txt")
sum = 0
trie = build_trie()

for line in input:
    word = trie
    numbers = []
    p1 = p2 = 0

    while p2 < len(line):
        if line[p2].isdigit():
            numbers.append(line[p2])
            p2 += 1
            p1 = p2
            word = trie
        
        else:
            if line[p2] in word.children:
                word = word.children[line[p2]]
                
                if word.number:
                    numbers.append(word.number)
                    p1 = p2
                    word = trie.children.get(line[p2], trie)

                p2 += 1

            else:
                word = trie
                p1 += 1
                p2 = p1
    lineadd = int(numbers[0]+numbers[-1])
    sum += lineadd
    print(f"process line {lineadd} {lineadd} {line.strip()}")

        
print(sum)