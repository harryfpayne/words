from words import *

def removeAnagrams(words):
	newWords = {}
	for word in words:
		if len("".join(set(word))) == 5:
			newWords[''.join(sorted(word))] = word
	return list(newWords.values())

words = removeAnagrams(allWords)
print(len(words))


def filterWords(word, remainingWords):
	letters = {}
	for l in word:
		letters[l] = True

	newWords = []
	for _word in remainingWords:
		foundLetterMatch = False
		for l in _word:
			if l in letters:
				foundLetterMatch = True
				break
		if not foundLetterMatch:
			newWords.append(_word)
	return newWords

def recurse(currentList, remainingWords):
	if len(currentList) > 4:
		print(currentList)

	for i, word in enumerate(remainingWords):
		recurse(currentList + [word], filterWords(word, remainingWords))
		if len(currentList) == 0:
			print(i)

recurse([], words)
