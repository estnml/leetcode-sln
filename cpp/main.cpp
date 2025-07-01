#include <iostream>
#include <vector>
#include <map>
#include <set>
#include <algorithm>
#include <string>

using namespace std;

class Solution
{
public:
	bool isPalindrome(string s)
	{
		int start_ptr = 0;
		int end_ptr = s.size() - 1;

		while (start_ptr < end_ptr)
		{

			while (start_ptr < end_ptr && !isalnum(s[start_ptr]))
			{
				start_ptr++;
			}

			while (start_ptr < end_ptr && !isalnum(s[end_ptr]))
			{
				end_ptr--;
			}

			if (tolower(s[start_ptr]) != tolower(s[end_ptr]))
			{
				return false;
			}

			start_ptr++;
			end_ptr--;
		}

		return true;
	}
	vector<int> twoSum(vector<int> &nums, int target)
	{
		map<int, int> num_map;
		vector<int> result_indexes;

		for (size_t i = 0; i < nums.size(); i++)
		{
			if (num_map.find(target - nums[i]) == num_map.end())
			{
				num_map.insert({nums[i], i});
			}
			else
			{
				auto x = num_map.find(target - nums[i]);

				if (x->second < i)
				{
					result_indexes.push_back(x->second);
					result_indexes.push_back(i);
				}
				else
				{
					result_indexes.push_back(i);
					result_indexes.push_back(x->second);
				}

				return result_indexes;
			}
		}
	}
	bool isAnagram(string s, string t)
	{

		if (s.length() != t.length())
		{
			return false;
		}

		map<char, int> scanned_chars;

		for (size_t i = 0; i < s.length(); i++)
		{
			scanned_chars[s[i]]++;
		} // n

		for (size_t i = 0; i < t.length(); i++)
		{
			if (scanned_chars.find(t[i]) != scanned_chars.end())
			{
				scanned_chars[t[i]]--;
			}
		} // m

		for (auto &pair : scanned_chars)
		{
			if (pair.second != 0)
			{
				return false;
			}
		}

		return true;
	}

	int maxProfit(vector<int> &prices)
	{
		int left_ptr = 0;
		int rigth_ptr = left_ptr + 1;
		int max = 0;

		while (rigth_ptr < prices.size())
		{
			if (prices[rigth_ptr] - prices[left_ptr] > max && left_ptr < rigth_ptr)
			{
				max = prices[rigth_ptr] - prices[left_ptr];
			}

			if (prices[left_ptr] > prices[rigth_ptr])
			{
				left_ptr = rigth_ptr++;
			}
			else
			{
				rigth_ptr++;
			}
		}

		return max;
	}

	bool hasDuplicate(vector<int> &nums)
	{
		map<int, int> num_occurence_map;
		for_each(nums.begin(), nums.end(), [&](int n) -> void
				 { num_occurence_map[n]++; });

		for (const auto &pair : num_occurence_map)
		{
			if (pair.second > 1)
			{
				return true;
			}
		}

		return false;
	}
};

int main(int argc, char **argv)
{
	vector<int> vec = {5, 1, 5, 6, 7, 1, 10};
	auto solver = new Solution();

	solver->maxProfit(vec);
	return 0;
}
