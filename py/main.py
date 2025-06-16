from typing import List
from math import e
from ext import facOfNRec, facOfNIterative, powerOfNRec, powerOfNIterative


class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        currentWordLength = 0  # 1
        wordLengths = []  # 1
        for ch in s:  # n
            if ch == " ":
                if currentWordLength != 0:
                    wordLengths.append(currentWordLength)
                    currentWordLength = 0
                continue
            currentWordLength += 1
        if currentWordLength > 0:
            wordLengths.append(currentWordLength)
        return wordLengths[-1]

    def twoSum(self, nums: List[int], target: int) -> List[int]:
        numDict = {}
        for i in range(len(nums)):
            if target - nums[i] in numDict:
                return [i, numDict.get(target - nums[i])]

            numDict[nums[i]] = i

    def reverseString(self, s: List[str]) -> None:
        left, right = 0, len(s) - 1
        while left < right:
            s[left], s[right] = s[right], s[left]
            left += 1
            right -= 1

    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False

        reverse = 0
        xcopy = x

        while x > 0:
            reverse = (reverse * 10) + (x % 10)
            x //= 10

        return reverse == xcopy


solver = Solution()
# solver.lengthOfLastWord("   fly me   to   the moon  ")
# solver.lengthOfLastWord("luffy is still joyboy")
# solver.twoSum([2, 7, 11, 15], 9)
# solver.reverseString(["H", "a", "n", "n", "a", "h"])
# print(solver.isPalindrome(111))

result = powerOfNIterative(e, 3)
print(result)
