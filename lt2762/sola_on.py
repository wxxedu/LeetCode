class Solution:
    def continuousSubarrays(self, nums: List[int]) -> int:
        start_idx = 0
        end_idx = 0
        min_val = nums[0]
        max_val = nums[0]

        
