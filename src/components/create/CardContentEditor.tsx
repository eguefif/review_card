import MDEditor from '@uiw/react-md-editor';

export default function CardContentEditor({ content, setContent }) {
  return (
  <MDEditor
      value={content}
      onChange={setContent}
      height='750px'
  />
  )
}
