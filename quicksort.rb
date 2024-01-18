def sort(numbers)
  pivot_index = numbers.size - 1
  pivot_value = numbers[pivot_index]

  # i = 0
  # j = pivot_index

  # loop do
  #   while numbers[i] < pivot_value do
  #     i += 1
  #   end

  #   while numbers[j] > pivot_value do
  #     j -= 1
  #   end

  #   if i >= j
  #     break
  #   else
  #     numbers[i], numbers[j] = numbers[j], numbers[i]
  #     i += 1
  #   end
  # end

  # numbers[i], numbers[pivot_index] = numbers[pivot_index], numbers[i]

  numbers.each_with_index do |number_i, i|
    if number_i >= pivot_value
      numbers.reverse_each.with_index do |number_j, j|
        if number_j <= pivot_value
          if i >= j
            numbers[pivot_index], numbers[i] = numbers[i], numbers[pivot_index]
            break
          else
            numbers[i], numbers[j] = numbers[j], numbers[i]
          end
        end
      end
    end
  end

  p numbers
end




sort([0, 5, 2, 1, 6, 3])



  # numbers.each_with_index do |number_i, i|
  #   if number_i >= pivot_value
  #     numbers.reverse_each.with_index do |number_j, j|
  #       if number_j <= pivot_value
  #         if i >= j
  #           # numbers[pivot_index], numbers[i] = numbers[i], numbers[pivot_index]
  #           break
  #         else
  #           numbers[i], numbers[j] = numbers[j], numbers[i]
  #         end
  #       end
  #     end
  #   end
  # end