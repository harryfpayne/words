package main

import (
	"fmt"
	"sort"
	"strings"
)

func removeAnagrams(words []string) []string {
	newWords := make(map[string]string)
	for _, word := range words {
		split := strings.Split(word, "")
		if len(removeDuplicates(split)) == len(word) {
			sort.Strings(split)
			newWords[strings.Join(split, "")] = word
		}
	}
	return mapToListValue[string](newWords)
}

func filterWords(word string, remainingWords []string) []string {
	letters := make(map[rune]struct{})
	for _, l := range word {
		letters[l] = struct{}{}
	}

	newWords := make([]string, 0)
	for _, remainingWord := range remainingWords {
		foundLetterMatch := false
		for _, l := range remainingWord {
			if _, ok := letters[l]; ok {
				foundLetterMatch = true
				break
			}
		}
		if !foundLetterMatch {
			newWords = append(newWords, remainingWord)
		}
	}

	return newWords
}

func recurse(currentWords []string, remainingWords []string){
	if len(currentWords) > 4 {
		fmt.Println(currentWords)
	}
	for i, word := range remainingWords {
		recurse(append(currentWords, word), filterWords(word, remainingWords))
		if len(currentWords) == 0 {
			fmt.Println(i)
		}
	}
}

func main() {
	words := removeAnagrams(AllWords)
	fmt.Println(len(words))
	recurse(make([]string, 0), words)
}

func mapToListValue[T any](m map[string]T) []T {
	l := make([]T, 0)
	for _, a := range m {
		l = append(l, a)
	}
	return l
}

func contains(s []string, e string) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}

func removeDuplicates(strList []string) []string {
	list := make([]string, 0)
	for _, item := range strList {
		if contains(list, item) == false {
			list = append(list, item)
		}
	}
	return list
}
