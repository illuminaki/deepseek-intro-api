#!/usr/bin/env ruby
# test-ruby.rb
# Prueba del skill DeepSeek en Ruby (sin dependencias externas)

require 'net/http'
require 'json'
require 'uri'

api_key = ENV['DEEPSEEK_API_KEY']

if !api_key || api_key.empty?
  puts "Error: DEEPSEEK_API_KEY no está configurada"
  exit 1
end

uri = URI('https://api.deepseek.com/chat/completions')

payload = {
  model: 'deepseek-chat',
  messages: [
    { role: 'system', content: 'You are a assistant angry and sarcastic' },
    { role: 'user', content: 'What is 2+2? dump' }
  ],
  stream: false
}.to_json

puts "[Ruby] Enviando petición a DeepSeek API..."
puts "Payload: #{payload}\n\n"

http = Net::HTTP.new(uri.host, uri.port)
http.use_ssl = true

request = Net::HTTP::Post.new(uri)
request['Content-Type'] = 'application/json'
request['Authorization'] = "Bearer #{api_key}"
request.body = payload

begin
  response = http.request(request)
  
  puts "HTTP Status: #{response.code}"
  
  if response.code == '200'
    data = JSON.parse(response.body)
    if data['choices'] && data['choices'].length > 0
      puts "✓ Respuesta exitosa:"
      puts data['choices'][0]['message']['content']
    else
      puts "✗ Respuesta inválida:"
      puts response.body
    end
  else
    puts "✗ Error HTTP #{response.code}:"
    puts response.body
  end
rescue => e
  puts "✗ Error: #{e.message}"
  exit 1
end
