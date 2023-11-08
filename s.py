from rouge_score import rouge_scorer

strings = [
    "this is a sentence to test this tool",
    "thus us i sentence to test thus took"
]

for string in strings:
    string = string[0].upper() + string[1:]
    string += "."

scorer = rouge_scorer.RougeScorer(['rouge1', 'rougeL'], use_stemmer=True)
scores = scorer.score(strings[0], strings[1])

print(scores)
