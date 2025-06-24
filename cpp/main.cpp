#include <iostream>
#include <vector>
#include <map>
#include <set>
#include <algorithm>
#include <string>

class Solution
{
public:
	bool hasDuplicate(std::vector<int> &nums)
	{
		std::map<int, int> num_occurence_map;
		std::for_each(nums.begin(), nums.end(), [&](int n) -> void
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
	return 0;
}
